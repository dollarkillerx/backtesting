//+------------------------------------------------------------------+
//|                                                      classic.mq5 |
//|                                  Copyright 2024, MetaQuotes Ltd. |
//|                                 https://github.com/dollarkillerx |
//+------------------------------------------------------------------+
#property copyright "Copyright 2024, MetaQuotes Ltd."
#property link      "https://github.com/dollarkillerx"
#property version   "1.00"
#include <trade.mqh>
//--- input parameters
input int      ma1=20;
input int      ma2=40;
input int      step=10;           // 步长
input double   initial_vol=0.01;  // 初始金额
input double   multiple=1.3;      // 倍数
input double   max_loss=1500.0;   // 最大亏损
input string   symbol="EURUSDc";  // 执行货币
input long     interval_time=15;  // 间隔时间(分)
input double   sink=30;           // 下落回撤
input double   sink_l1=30;        // >=40回撤到
input double   sink_l2=60;        // >=80回撤到
input int      magic=6969; // magic

EasyUtils easyUtils(symbol, magic,20);

//+------------------------------------------------------------------+
//| Expert initialization function                                   |
//+------------------------------------------------------------------+
int OnInit()
  {
//--- create timer
   EventSetTimer(1);

// Print("tj：",SymbolInfoDouble(symbol, SYMBOL_POINT));
//---
   return(INIT_SUCCEEDED);
  }
//+------------------------------------------------------------------+
//| Expert deinitialization function                                 |
//+------------------------------------------------------------------+
void OnDeinit(const int reason)
  {
//--- destroy timer
   EventKillTimer();

  }
//+------------------------------------------------------------------+
//| Expert tick function                                             |
//+------------------------------------------------------------------+
void OnTick()
  {

  }
//+------------------------------------------------------------------+
//| Timer function                                                   |
//+------------------------------------------------------------------+
void OnTimer()
  {
   Print("la1");
   close_server();
   ENUM_POSITION_TYPE direction = POSITION_TYPE_BUY;
   int before_vol = 0; // 上一个订单的vol

// 0. 获得最新的order记录
   SymbolInfo last_position();
   last_position.GetLastPosition();
   if(PositionsTotal() != 0)
     {

      before_vol = last_position.last_position.volume;
      direction = last_position.last_position.direction;

      // 如果最近最后一个订单 却时间小于间隔时间 就跳过
      long  serverTime = TimeCurrent();
      if(MinutesToMilliseconds(interval_time) > serverTime * 1000 - last_position.last_position.time_msc)
        {
         return;
        }

      // 价格间隔

      if(MathAbs(SymbolInfoDouble(symbol, SYMBOL_ASK) - last_position.last_position.price_open) < SymbolInfoDouble(symbol, SYMBOL_POINT) * 10 * step)
        {
         Print("MathAbs:",MathAbs(SymbolInfoDouble(symbol, SYMBOL_ASK) - last_position.last_position.price_open));
         Print("SYMBOL_ASK:",SymbolInfoDouble(symbol, SYMBOL_ASK));
         Print("price_open:",last_position.last_position.price_open);
         return;
        }
     }

   else
     {
      // 1. 获取日线ma
      SymbolInfo day_ma20();
      day_ma20.ma(symbol, PERIOD_D1, 20,0, MODE_EMA, PRICE_CLOSE);
      SymbolInfo day_ma40();
      day_ma40.ma(symbol, PERIOD_D1, 40,0, MODE_EMA, PRICE_CLOSE);
      // 如果20>40  buy 方向
      // 20<40 sell 方向
      if(day_ma20.ima[0] > day_ma40.ima[0])
        {
         direction = POSITION_TYPE_BUY;
        }
      else
        {
         direction = POSITION_TYPE_SELL;
        }
     }

// 加仓入局
   double new_volume = initial_vol;
   Print("vol: ",last_position.last_position.volume);
   if(last_position.last_position.volume == 0)
     {
      // Alert("this1");
      new_volume = initial_vol;
     }
   else
     {
      // Alert("this2",new_volume);
      // Alert("last_position",last_position.last_position.volume);
      new_volume = new_volume + last_position.last_position.volume;
     }

// 下单
   if(direction == POSITION_TYPE_BUY)
     {
      easyUtils.Buy(symbol,new_volume);
      Print("Buy ",  new_volume);
     }
   else
     {
      easyUtils.Sell(symbol,new_volume);
      Print("Sell ",  new_volume);
     }
  }

double globalHight;
void close_server()
  {
   if(PositionsTotal() == 0)
     {
      return;
     }
   if(globalHight < AccountInfoDouble(ACCOUNT_PROFIT))
     {
      globalHight = AccountInfoDouble(ACCOUNT_PROFIT);
     }

   if(AccountInfoDouble(ACCOUNT_PROFIT) > 2)
     {
      if(globalHight - AccountInfoDouble(ACCOUNT_PROFIT) >= sink)
        {
         easyUtils.CloseAll(symbol);
         return;
        }
      if(globalHight > 80 && AccountInfoDouble(ACCOUNT_PROFIT) <= sink_l2)
        {
         easyUtils.CloseAll(symbol);
         return;
        }
      if(globalHight > 40 && AccountInfoDouble(ACCOUNT_PROFIT) <= sink_l1)
        {
         easyUtils.CloseAll(symbol);
         return;
        }

     }
  }
//+------------------------------------------------------------------+
//| Trade function                                                   |
//+------------------------------------------------------------------+
void OnTrade()
  {
//---

  }
//+------------------------------------------------------------------+
//| TradeTransaction function                                        |
//+------------------------------------------------------------------+
void OnTradeTransaction(const MqlTradeTransaction& trans,
                        const MqlTradeRequest& request,
                        const MqlTradeResult& result)
  {
//---

  }
//+------------------------------------------------------------------+
