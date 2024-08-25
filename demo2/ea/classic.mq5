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
input double   multiple=1.3;      // 倍数
input double   max_loss=1500.0;   // 最大亏损
input string   symbol="EURUSDm";  // 执行货币
//+------------------------------------------------------------------+
//| Expert initialization function                                   |
//+------------------------------------------------------------------+
int OnInit()
  {
//--- create timer
   EventSetTimer(60);
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
//---
  // 0. 获得最新的order记录



   // 1. 获取日线ma
   SymbolInfo day_ma20();
   day_ma20.ma(symbol, PERIOD_D1, 20,0, MODE_EMA, PRICE_CLOSE);
   SymbolInfo day_ma40();
   day_ma40.ma(symbol, PERIOD_D1, 40,0, MODE_EMA, PRICE_CLOSE);
   day_ma40.GetLastPosition();
   Alert(day_ma40.last_position.ticket);
  // 如果20>40  buy 方向
  // 20<40 sell 方向
  }
//+------------------------------------------------------------------+
//| Timer function                                                   |
//+------------------------------------------------------------------+
void OnTimer()
  {
//---

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
