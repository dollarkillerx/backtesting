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
input double   initial_vol=0.1;  // 初始金额
input double   multiple=1.3;      // 倍数
input double   max_loss=1500.0;   // 最大亏损
input string   symbol="EURUSD";  // 执行货币
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
   FileWrite(file_handle, "Time,SL,TP,Type,Comment,Position,Position_BY,Volume,Ask,Bid,Price");

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
         easyUtils.Buy(symbol, initial_vol,0,15);
        }
      else
        {
         direction = POSITION_TYPE_SELL;
         easyUtils.Sell(symbol, initial_vol,0,15);
        }
      return;
     }

// 多个订单获取上次的订单
   SymbolInfo last_position();
   last_position.GetLastPosition();

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


   if(AccountInfoDouble(ACCOUNT_PROFIT) > 10)
     {
      return;
     }
   double new_volume = initial_vol + last_position.last_position.volume;

   if(last_position.last_position.direction == POSITION_TYPE_BUY)
     {
      easyUtils.Buy(symbol, new_volume * 1.3,15);
     }
   else
     {
      easyUtils.Sell(symbol, new_volume * 1.3,15);
     }


  }


double globalHight;
void close_server()
  {

   if(PositionsTotal() == 0)
     {
      return;
     }


// 如果都盈利
   int profitable_quantity = 0;
   ulong last_time = 0;
   if(PositionsTotal() > 0)
     {
      for(int i=PositionsTotal()-1;i>=0;i--)
        {
         ulong ticket = PositionGetTicket(i);
         if(ticket > 0)
           {
            if(PositionGetTicket(i) == ticket)
              {
               if(i==PositionsTotal()-1)
                 {
                  last_time = PositionGetInteger(POSITION_TIME_MSC);
                 }
               if(PositionGetDouble(POSITION_PROFIT) > 10)
                 {
                  profitable_quantity+=1;
                 }
              }
           }
        }
     }

// 如果都盈利且时间> 1h 就关闭
   if(profitable_quantity == PositionsTotal() && AccountInfoDouble(ACCOUNT_PROFIT) > 10)
     {
      if(TimeCurrent() * 1000 - last_time > 60*60*1000)
        {
         easyUtils.CloseAll(symbol);
         globalHight = 0;
         return;
        }
     }

   if(globalHight < AccountInfoDouble(ACCOUNT_PROFIT))
     {
      globalHight = AccountInfoDouble(ACCOUNT_PROFIT);
     }


   if(AccountInfoDouble(ACCOUNT_PROFIT) > 10)
     {
      if(globalHight - AccountInfoDouble(ACCOUNT_PROFIT) >= sink)
        {
         easyUtils.CloseAll(symbol);
         globalHight = 0;
         return;
        }
      if(globalHight > 80 && AccountInfoDouble(ACCOUNT_PROFIT) <= sink_l2)
        {
         easyUtils.CloseAll(symbol);
         globalHight = 0;
         return;
        }
      if(globalHight > 40 && AccountInfoDouble(ACCOUNT_PROFIT) <= sink_l1)
        {
         easyUtils.CloseAll(symbol);
         globalHight = 0;
         return;
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
void OnTradeTransaction(const MqlTradeTransaction &trans,const MqlTradeRequest &request,const MqlTradeResult &result)
  {
// Check if the transaction type is TRADE_TRANSACTION_DEAL_ADD (a new deal has been added)
// FileWrite(file_handle, "Time,SL,TP,Type,Comment,Position,Position_BY,Volume,Ask,Bid,Price");
if (result.volume == 0) {
return;
}
   FileWrite(file_handle, StringFormat("%d,%.4f,%.4f,%d,%d,%d,%.4f,%.4f,%.4f,%.4f",
                                       TimeCurrent(),trans.price_sl, trans.price_tp, trans.order_type,trans.position,trans.position_by,result.volume,result.ask,result.bid,result.price));
  }

//+------------------------------------------------------------------+
