//+------------------------------------------------------------------+
//|                                                      classic.mq5 |
//|                                  Copyright 2024, MetaQuotes Ltd. |
//|                                 https://github.com/dollarkillerx |
//+------------------------------------------------------------------+
#property copyright "Copyright 2024, MetaQuotes Ltd."
#property link      "https://github.com/dollarkillerx"
#property version   "1.00"
#include <trade.mqh>

// #import "rustdll.dll"
// void rs_log(string _version, int order_total,double volume,double profit,int time);
// #import

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

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
EasyUtils easyUtils(symbol, magic,20);
int file_handle = INVALID_HANDLE;

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
int OnInit()
  {
//--- create timer
   EventSetTimer(60);
   file_handle = FileOpen("example.csv", FILE_WRITE | FILE_CSV | FILE_ANSI);
   FileWrite(file_handle, "Total,Vol,Profile,Time");
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
   FileClose(file_handle);
  }
//+------------------------------------------------------------------+
//| Expert tick function                                             |
//+------------------------------------------------------------------+
void OnTick()
  {
//---
   close_server();
// log_info();
   ENUM_POSITION_TYPE direction = POSITION_TYPE_BUY;
   int before_vol = 0; // 上一个订单的vol

   if(PositionsTotal() == 0)
     {
      double ma1[];
      double ma2[];
      int header1 = iMA(symbol, PERIOD_D1, 20,0,MODE_EMA, PRICE_CLOSE);
      int header2 = iMA(symbol, PERIOD_D1, 40,0,MODE_EMA, PRICE_CLOSE);
      ArraySetAsSeries(ma1, true);
      ArraySetAsSeries(ma2, true);
      CopyBuffer(header1,0,0,30,ma1);
      CopyBuffer(header2,0,0,30,ma2);
      IndicatorRelease(header1);
      IndicatorRelease(header2);
      if(ma1[0] > ma2[0])
        {
         direction = POSITION_TYPE_BUY;
         easyUtils.Buy(symbol, initial_vol);
        }
      else
        {
         direction = POSITION_TYPE_SELL;
         easyUtils.Sell(symbol, initial_vol);
        }
      return;
     }

// 多个订单获取上次的订单
   SymbolInfo last_position();
   last_position.GetLastPosition();
// Print("ticket: ", last_position.last_position.ticket);
// Print("ticket1: ", last_position.last_position.direction);
// Print("ticket2: ", last_position.last_position.volume);
// Print("ticket3: ", last_position.last_position.price_open);
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

// 过滤历史是否和当前位置重叠
   if(PositionsTotal() > 0)
     {
      for(int i=PositionsTotal()-1;i>=0;i--)
        {
         ulong ticket = PositionGetTicket(i);
         if(ticket > 0)
           {
            if(PositionGetTicket(i) == ticket)
              {
               double op_price = PositionGetDouble(POSITION_PRICE_OPEN);
               if(MathAbs(op_price - SymbolInfoDouble(symbol, SYMBOL_ASK)) < SymbolInfoDouble(symbol, SYMBOL_POINT) * 8 * step)
                 {

                  return;
                 }
              }
           }
        }
     }

   double ma1[];
   double ma2[];
   int header1 = iMA(symbol, PERIOD_D1, 20,0,MODE_EMA, PRICE_CLOSE);
   int header2 = iMA(symbol, PERIOD_D1, 40,0,MODE_EMA, PRICE_CLOSE);
   ArraySetAsSeries(ma1, true);
   ArraySetAsSeries(ma2, true);
   CopyBuffer(header1,0,0,30,ma1);
   CopyBuffer(header2,0,0,30,ma2);
   IndicatorRelease(header1);
   IndicatorRelease(header2);
   if(ma1[0] > ma2[0])
     {
      direction = POSITION_TYPE_BUY;
     }
   else
     {
      direction = POSITION_TYPE_SELL;
     }

   ArrayFree(ma1);
   ArrayFree(ma2);

   double new_volume = initial_vol + last_position.last_position.volume;
   Print("new_volume", new_volume);
   new_volume = new_volume * 2;
   Print("direction", last_position.last_position.direction);
   if(direction == POSITION_TYPE_BUY)
     {
      easyUtils.Buy(symbol, new_volume);
     }
   else
     {
      easyUtils.Sell(symbol, new_volume);
     }


  }


double globalHight;
void close_server()
  {

   if(AccountInfoDouble(ACCOUNT_PROFIT) < 0)
     {
      if(MathAbs(AccountInfoDouble(ACCOUNT_PROFIT)) > 1500)
        {
           Print("rj: ",MathAbs(AccountInfoDouble(ACCOUNT_PROFIT)));
         easyUtils.CloseAll(symbol);
         globalHight = 0;
         return;
        }
     }

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


      if (AccountInfoDouble(ACCOUNT_PROFIT) > 100) {
         if(globalHight - AccountInfoDouble(ACCOUNT_PROFIT) >= (globalHight/3) * 2)
           {
            easyUtils.CloseAll(symbol);
            globalHight = 0;
            return;
           }
      }


     }


  }

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
double gl_vol;
string gl_key;
void log_info()
  {
   double vol;

   int total = PositionsTotal();
   if(total ==0)
     {
      return;
     }
   if(total > 0)
     {
      for(int i=total-1;i>=0;i--)
        {
         ulong ticket = PositionGetTicket(i);
         if(ticket > 0)
           {
            if(PositionGetTicket(i) == ticket)
              {
               vol += PositionGetDouble(POSITION_VOLUME);
              }
           }
        }
     }

// rs_log("v1",total,vol, AccountInfoDouble(ACCOUNT_PROFIT),TimeCurrent());
   string vkey =  StringFormat("%.4f-%d", vol, total);
   if(vkey != gl_key)
     {
      gl_key = vkey;
      FileWrite(file_handle, StringFormat("%d,%.2f,%.2f,%d", total, vol,AccountInfoDouble(ACCOUNT_PROFIT), TimeCurrent())); // total,",", vol, ",",AccountInfoDouble(ACCOUNT_PROFIT), ",",TimeCurrent()
      FileFlush(file_handle);
     }
   else
     {
      if(MathAbs(AccountInfoDouble(ACCOUNT_PROFIT)-gl_vol) > 5)
        {
         gl_vol = AccountInfoDouble(ACCOUNT_PROFIT);
         FileWrite(file_handle, StringFormat("%d,%.2f,%.2f,%d", total, vol,AccountInfoDouble(ACCOUNT_PROFIT), TimeCurrent())); // total,",", vol, ",",AccountInfoDouble(ACCOUNT_PROFIT), ",",TimeCurrent()
         FileFlush(file_handle);
        }
     }
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
