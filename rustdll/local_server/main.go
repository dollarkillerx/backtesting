package main

import (
	"fmt"
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
	Sl        float64 `json:"sl"`
	Tp        float64 `json:"tp"`
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

var orders = make([]Order, 0)
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
	engine := gin.Default()

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
		orders = append(orders, Order{
			Id:        int64(id),
			Price:     payload.Price,
			Volume:    payload.InitVolume,
			OrderType: payload.IsBuy,
			Time:      payload.Time,
			Sl:        0,
			Tp:        payload.Price + 0.0001*15.0,
		})

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

		mu.Lock()
		defer mu.Unlock()
		latestOrder := getLatestOrder()
		// 加仓逻辑
		// 1. 价格必须大于 step 步长
		if math.Abs(payload.Ask-latestOrder.Price) < payload.Step {
			c.JSON(200, OrderResponse{
				OrderId: int64(0),
			})
			return
		}
		// 2. 时间必须大于 interval_time 间隔时间
		if payload.IntervalTime*60 > payload.Time-latestOrder.Time {
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
				Sl:        payload.Ask + 0.0001*15.0,
				Tp:        0,
			})
		case 0:
			orders = append(orders, Order{
				Id:        int64(id),
				Price:     payload.Bid,
				Volume:    newVolume,
				OrderType: latestOrder.OrderType,
				Time:      payload.Time,
				Sl:        payload.Bid - 0.0001*15.0,
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
			c.JSON(400, gin.H{
				"message": "invalid payload",
			})
			return
		}

		mu.Lock()
		defer mu.Unlock()
	})

	engine.POST("/close_all", func(c *gin.Context) {

	})

	engine.GET("/test/:key", func(c *gin.Context) {
		key := c.Param("key")
		fmt.Println(key)
		c.JSON(200, gin.H{
			"message": "success",
		})
	})

	engine.Run(":8181")
}
