package main

import (
	"fmt"
	"log"
	"math"
	"math/rand"
	"sort"
	"sync"
	"time"

	"github.com/gin-gonic/gin"
)

var (
	usedNumbers = make(map[int]bool)
	muRounds    sync.Mutex
)

func init() {
	rand.NewSource(time.Now().UnixNano())
}

func generateUniqueRandomInt(min, max int) int {
	muRounds.Lock()
	defer muRounds.Unlock()

	for {
		num := rand.Intn(max-min+1) + min
		if !usedNumbers[num] {
			usedNumbers[num] = true
			return num
		}
	}
}

type Order struct {
	Id        int64   `json:"id"`
	Price     float64 `json:"price"`
	Volume    float64 `json:"volume"`
	Sl        float64 `json:"sl"`         // 止损
	Tp        float64 `json:"tp"`         // 止盈
	OrderType int64   `json:"order_type"` // 1:buy, 0:sell
	Time      int64   `json:"time"`
}

type InitialOrderPayload struct {
	Price      float64 `json:"price"`
	InitVolume float64 `json:"init_volume"`
	IsBuy      int64   `json:"is_buy"`
	Time       int64   `json:"time"`
}

type OrderResponse struct {
	OrderId int64 `json:"order_id"`
}

type AddOrderPayload struct {
	Ask           float64 `json:"ask"`
	Bid           float64 `json:"bid"`
	Time          int64   `json:"time"`
	Step          float64 `json:"step"`
	IntervalTime  int64   `json:"interval_time"`
	InitialVolume float64 `json:"initial_volume"`
}

type AutoClosePayload struct {
	Ask float64 `json:"ask"`
	Bid float64 `json:"bid"`
}

type CloseAllPayload struct {
	Ask   float64 `json:"ask"`
	Bid   float64 `json:"bid"`
	Time  int64   `json:"time"`
	Sink  float64 `json:"sink"`
	Sink1 float64 `json:"sink1"`
	Sink2 float64 `json:"sink2"`
}

var orders = make([]Order, 0)
var high = 0.00
var mu sync.Mutex

// 获取最新的订单
func getLatestOrder() *Order {
	if len(orders) == 0 {
		return nil
	}

	// 按时间倒序排序
	sort.SliceStable(orders, func(i, j int) bool {
		return orders[i].Time > orders[j].Time
	})

	// 返回最新的订单
	return &orders[0]
}

// 根据 id 查找订单
func findOrderById(id int64) *Order {
	for _, order := range orders {
		if order.Id == id {
			return &order
		}
	}

	// 如果未找到订单，返回 nil
	return nil
}

func main() {
	//engine := gin.Default()
	engine := gin.New()

	engine.POST("/initial_order", func(c *gin.Context) {
		var payload InitialOrderPayload
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(400, gin.H{
				"message": "invalid payload",
			})
			return
		}
		mu.Lock()
		defer mu.Unlock()

		id := generateUniqueRandomInt(1, 1000000)
		orders = make([]Order, 0)
		if payload.IsBuy == 1 {
			orders = append(orders, Order{
				Id:        int64(id),
				Price:     payload.Price,
				Volume:    payload.InitVolume,
				OrderType: payload.IsBuy,
				Time:      payload.Time,
				Sl:        0,
				Tp:        payload.Price + 0.0001*15.0,
			})
		} else {
			orders = append(orders, Order{
				Id:        int64(id),
				Price:     payload.Price,
				Volume:    payload.InitVolume,
				OrderType: payload.IsBuy,
				Time:      payload.Time,
				Sl:        0,
				Tp:        payload.Price - 0.0001*15.0,
			})
		}

		c.JSON(200, OrderResponse{
			OrderId: int64(id),
		})
	})

	engine.POST("/add_order", func(c *gin.Context) {
		var payload AddOrderPayload
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(400, gin.H{
				"message": "invalid payload",
			})
			return
		}

		payload.IntervalTime = 15

		mu.Lock()
		defer mu.Unlock()

		fmt.Println("total", len(orders))

		latestOrder := getLatestOrder()
		if latestOrder == nil {
			log.Println("????????????????")
			c.JSON(200, OrderResponse{
				OrderId: int64(0),
			})
			return
		}
		// 加仓逻辑
		// 1. 价格必须大于 step 步长
		if math.Abs(payload.Ask-latestOrder.Price) < payload.Step*0.0001 {
			//log.Printf("A: ask: %.4f  price: %.4f, ex: %.4f steo: %.4f", payload.Ask, latestOrder.Price, payload.Ask-latestOrder.Price, payload.Step*0.0001)
			c.JSON(200, OrderResponse{
				OrderId: int64(0),
			})
			return
		}
		// 2. 时间必须大于 interval_time 间隔时间
		if payload.IntervalTime*60 > payload.Time-latestOrder.Time {
			//log.Printf("B: IntervalTime %d  time: %d, ex: %d steo: %d", payload.IntervalTime*60, payload.Time, latestOrder.Time, payload.Time-latestOrder.Time)
			c.JSON(200, OrderResponse{
				OrderId: int64(0),
			})
			return
		}

		// 统计盈亏
		var profit float64 = 0
		for _, item := range orders {
			if item.OrderType == 1 {
				// buy
				profit += (payload.Bid - item.Price) * item.Volume
			} else {
				// sell
				profit += (item.Price - payload.Ask) * item.Volume
			}
		}

		// 盈利不加仓
		if profit > 1 {
			c.JSON(200, OrderResponse{
				OrderId: int64(0),
			})
			return
		}

		var newVolume = latestOrder.Volume + payload.InitialVolume
		var id = generateUniqueRandomInt(1, 1000000)
		switch latestOrder.OrderType {
		case 1:
			orders = append(orders, Order{
				Id:        int64(id),
				Price:     payload.Ask,
				Volume:    newVolume,
				OrderType: latestOrder.OrderType,
				Time:      payload.Time,
				Sl:        payload.Bid - 0.0001*15.0,
				Tp:        0,
			})
		case 0:
			orders = append(orders, Order{
				Id:        int64(id),
				Price:     payload.Bid,
				Volume:    newVolume,
				OrderType: latestOrder.OrderType,
				Time:      payload.Time,
				Sl:        payload.Ask + 0.0001*15.0,
				Tp:        0,
			})
		}

		c.JSON(200, OrderResponse{
			OrderId: int64(id),
		})
		return
	})

	engine.POST("/get_order_position_type", func(c *gin.Context) {
		var payload OrderResponse
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(400, gin.H{
				"message": "invalid payload",
			})
			return
		}

		mu.Lock()
		defer mu.Unlock()

		order := findOrderById(payload.OrderId)
		if order != nil {
			c.JSON(200, gin.H{
				"position_type": order.OrderType,
			})
			return
		}

		c.JSON(200, gin.H{
			"position_type": -1,
		})
	})

	engine.POST("/get_order_volume", func(c *gin.Context) {
		var payload OrderResponse
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(400, gin.H{
				"message": "invalid payload",
			})
			return
		}

		mu.Lock()
		defer mu.Unlock()

		order := findOrderById(payload.OrderId)
		if order != nil {
			c.JSON(200, gin.H{
				"volume": order.Volume,
			})
			return
		}

		c.JSON(200, gin.H{
			"volume": -1,
		})
	})

	engine.POST("/auto_close", func(c *gin.Context) {
		var payload AutoClosePayload
		if err := c.ShouldBindJSON(&payload); err != nil {
			fmt.Println("err: ", err)
			c.JSON(400, gin.H{
				"message": "invalid payload",
			})
			return
		}

		mu.Lock()
		defer mu.Unlock()
		for i := 0; i < len(orders); i++ {
			order := orders[i]

			// Check for buy orders
			if order.OrderType == 1 {
				if order.Sl != 0 {
					if order.Sl >= payload.Bid {
						fmt.Println("1close-----", order, "  ", payload.Ask, "   ", payload.Bid)
						orders = append(orders[:i], orders[i+1:]...)
						c.JSON(200, OrderResponse{
							OrderId: order.Id,
						})
						return
					}
				}
				if order.Tp != 0 {
					if order.Tp <= payload.Bid {
						fmt.Println("2close-----", order, "  ", payload.Ask, "   ", payload.Bid)
						orders = append(orders[:i], orders[i+1:]...)
						c.JSON(200, OrderResponse{
							OrderId: order.Id,
						})
						return
					}
				}
			}

			// Check for sell orders
			if order.OrderType == 0 {
				if order.Sl != 0 {
					if order.Sl <= payload.Ask {
						fmt.Println("3close-----", order, "  ", payload.Ask, "   ", payload.Bid)
						orders = append(orders[:i], orders[i+1:]...)
						c.JSON(200, OrderResponse{
							OrderId: order.Id,
						})
						return
					}
				}
				if order.Tp != 0 {
					if order.Tp >= payload.Ask {
						fmt.Println("4close-----", order, "  ", payload.Ask, "   ", payload.Bid, " ", order.Id)
						orders = append(orders[:i], orders[i+1:]...)
						c.JSON(200, OrderResponse{
							OrderId: order.Id,
						})
						return
					}
				}
			}
		}

		c.JSON(200, OrderResponse{
			OrderId: int64(0),
		})
	})

	engine.POST("/close_all", func(c *gin.Context) {
		var payload CloseAllPayload
		if err := c.ShouldBindJSON(&payload); err != nil {
			c.JSON(400, gin.H{
				"message": "invalid payload",
			})
			return
		}

		mu.Lock()
		defer mu.Unlock()

		if getLatestOrder() == nil {
			c.JSON(200, gin.H{
				"close_all": true,
			})
			return
		}

		// 统计所有订单当前盈利
		totalProfit := 0.0                 // 总盈利
		totalOrder := 0                    // 总订单
		profitable_quantity := 0           // 盈利订单数量
		last_time := getLatestOrder().Time // 最后一笔订单时间

		for i := 0; i < len(orders); i++ {
			order := orders[i]
			totalOrder++
			switch order.OrderType {
			case 1: // buy
				totalProfit = totalProfit + (payload.Bid-order.Price)*order.Volume
				if (payload.Bid-order.Price)*order.Volume > 1 {
					profitable_quantity++
				}
			case 0: // sell
				totalProfit = totalProfit + (order.Price-payload.Ask)*order.Volume
				if (order.Price-payload.Ask)*order.Volume > 1 {
					profitable_quantity++
				}
			}
		}

		// 如果都盈利且时间超过1h，清空所有订单
		if totalOrder == profitable_quantity {
			if totalProfit > 1 && totalOrder > 0 {
				if payload.Time-last_time > 60*60 {
					// 清空所有订单
					high = 0
					orders = make([]Order, 0)
					log.Println("close all------------")
					c.JSON(200, gin.H{
						"close_all": true,
					})
					return
				}
			}
		}

		if totalProfit > high {
			high = totalProfit
		}

		if high > 2 {
			if (high-totalProfit >= payload.Sink) ||
				(high > 80 && totalProfit <= payload.Sink2) ||
				(high > 40 && totalProfit <= payload.Sink1) {
				// 清空所有订单
				high = 0
				orders = make([]Order, 0)
				log.Println("close all------------")
				c.JSON(200, gin.H{
					"close_all": true,
				})
				return
			}
		}

		c.JSON(200, gin.H{
			"close_all": false,
		})
	})

	engine.GET("/test/:key", func(c *gin.Context) {
		key := c.Param("key")
		fmt.Println("----------------------------------")
		fmt.Println(key)
		fmt.Println("----------------------------------")
		c.JSON(200, gin.H{
			"message": "success",
		})
	})

	engine.Run("127.0.0.1:8181")
}
