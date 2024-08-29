
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
input string   symbol="EURUSD";  // 执行货币
input long     interval_time=15;  // 间隔时间(分)
input double   sink=30;           // 下落回撤
input double   sink_l1=30;        // >=40回撤到
input double   sink_l2=60;        // >=80回撤到
input int      magic=6969; // magic

#import "rustdll.dll"
int initial_order(double price, double init_volume, int is_buy, int time);
int add_order(double ask_price, double bin_price, int time, double step, int interval_time, double initial_vol);
int get_order_position_type(int id);
double get_order_volume(int id);
int auto_close(double ask, double bid);
bool close_all(double ask, double bid, int time, double sink, double sink1, double sink2);
#import
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
   Print("cr",IntegerToString(7848484819));
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

// FileClose(file_handle);
  }
//+------------------------------------------------------------------+
//| Expert tick function                                             |
//+------------------------------------------------------------------+
void OnTick()
  {

   Print("rk:",SymbolInfoDouble(symbol, SYMBOL_ASK));

   Print("bid:",SymbolInfoDouble(symbol, SYMBOL_BID));
   long  serverTime = TimeCurrent();
   close_server();
   my_auto_close();

   ENUM_POSITION_TYPE direction = POSITION_TYPE_BUY;
   int before_vol = 0; // 上一个订单的vol

   if(PositionsTotal() == 0)
     {
      double ma1[];
      double ma2[];
      int header1 = iMA(symbol, PERIOD_H2, 20,0,MODE_EMA, PRICE_CLOSE);
      int header2 = iMA(symbol, PERIOD_H2, 40,0,MODE_EMA, PRICE_CLOSE);
      ArraySetAsSeries(ma1, true);
      ArraySetAsSeries(ma2, true);
      CopyBuffer(header1,0,0,30,ma1);
      CopyBuffer(header2,0,0,30,ma2);
      IndicatorRelease(header1);
      IndicatorRelease(header2);
      if(ma1[0] < ma2[0])
        {
         direction = POSITION_TYPE_BUY;

         int cid = initial_order(SymbolInfoDouble(symbol, SYMBOL_ASK), initial_vol, 1, serverTime);

          Print("initial_order:",IntegerToString(cid));
         easyUtils.Buy(symbol, initial_vol,0,0,IntegerToString(cid));
        }
      else
        {
         direction = POSITION_TYPE_SELL;
         int cid = initial_order(SymbolInfoDouble(symbol, SYMBOL_BID), initial_vol, 0, serverTime);

          Print("initial_order:",IntegerToString(cid));
         easyUtils.Sell(symbol, initial_vol,0,0,IntegerToString(cid));
        }
      return;
     }


   int oid = add_order(SymbolInfoDouble(symbol, SYMBOL_ASK), SymbolInfoDouble(symbol, SYMBOL_BID),serverTime,step,interval_time,initial_vol);
   int oid_type = get_order_position_type(oid);
   double oid_val = get_order_volume(oid);
   if(oid_type == 1)
     {
      // buy
      easyUtils.Buy(symbol, oid_val,0,0,IntegerToString(oid));
     }
   else
     {
      // sell
      easyUtils.Sell(symbol, oid_val,0,0,IntegerToString(oid));
     }
  }

// 方法1: 将整数转换为字符串
string IntToString(int value)
  {
   return IntegerToString(value);
  }

// 方法2: 将字符串转换为整数
int StringToInt(string value)
  {
   return StringToInteger(value);
  }

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void my_auto_close()
  {
   while(true)
     {
      int cid = auto_close(SymbolInfoDouble(symbol, SYMBOL_ASK), SymbolInfoDouble(symbol, SYMBOL_BID));
      if(cid == 0)
        {
         return;
        }
      close_order_by_comment(cid);
     }
  }

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
void close_order_by_comment(int cid)
  {
   if(cid == 0)
     {
      return;
     }
   string comment = IntegerToString(cid);
   int total = PositionsTotal();
   if(total <=0)
     {
      return;
     }
   for(int i=total-1;i>=0;i--)
     {
      ulong ticket = PositionGetTicket(i);
      if(ticket > 0)
        {
         if(PositionGetTicket(i) == ticket)
           {
            if(comment == PositionGetString(POSITION_COMMENT))
              {
               easyUtils.CloseOrderByTicket(ticket);
               return;
              }
           }
        }
     }
  }

double globalHight;
void close_server()
  {

   bool clo = close_all(SymbolInfoDouble(symbol, SYMBOL_ASK), SymbolInfoDouble(symbol, SYMBOL_BID),TimeCurrent(), sink, sink_l1,sink_l2);
   if(clo)
     {
      easyUtils.CloseAll(symbol);
      globalHight = 0;
      return;
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
