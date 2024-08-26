//+------------------------------------------------------------------+
//|                                                        trade.mqh |
//|                                  Copyright 2024, MetaQuotes Ltd. |
//|                                 https://github.com/dollarkillerx |
//+------------------------------------------------------------------+
#property copyright "Copyright 2024, MetaQuotes Ltd."
#property link      "https://github.com/dollarkillerx"

// 账户基本信息
struct AccountInfoPayload
  {
   string            account_name;     // 账户名称
   long              account;            // 账户登录ID
   string            account_server;   // 交易服务器
   string            account_company;  // 提供账户公司名称
   bool              ea_ok;              // EA 是否可用

   double            balance;          // 余额
   double            profit;           // 利润
   double            margin;           // 使用保证金
   long              level;              // 杠杆
  };

// Symbol货币基础信息
struct SymbolInfoPayload
  {
   string            symbol;           // 当前图表货币名称
   ENUM_TIMEFRAMES   period;  // 当前时间表
   int               digits;              // 当前货币对交易小数点
   double            point;            // 返回当前交易品种大小点（最小交易量）
   long              spread;             // 点差
   double            min_volume;       // 最低持仓数量
  };

// 工具类定义
class EasyUtils
  {

public:
   // 获取账户信息的方法
   static AccountInfoPayload GetAccountInfo()
     {
      AccountInfoPayload info;  // 创建结构体实例

      // 设置结构体的各个属性值
      info.account_name = AccountInfoString(ACCOUNT_NAME);
      info.account = AccountInfoInteger(ACCOUNT_LOGIN);
      info.account_server = AccountInfoString(ACCOUNT_SERVER);
      info.account_company = AccountInfoString(ACCOUNT_COMPANY);

      info.balance = AccountInfoDouble(ACCOUNT_BALANCE);
      info.profit  = AccountInfoDouble(ACCOUNT_PROFIT);
      info.margin  = AccountInfoDouble(ACCOUNT_MARGIN);
      info.level   = AccountInfoInteger(ACCOUNT_LEVERAGE);

      info.ea_ok = AccountInfoInteger(ACCOUNT_TRADE_EXPERT) == 1;

      return info;  // 返回结构体实例
     }

   // 获取货币符号信息的方法
   static SymbolInfoPayload GetSymbolInfo(string symbol)
     {
      SymbolInfoPayload info;

      info.symbol = symbol;
      info.period = Period();
      info.digits = SymbolInfoInteger(symbol, SYMBOL_DIGITS);
      info.point = SymbolInfoDouble(symbol, SYMBOL_POINT);
      info.spread = SymbolInfoInteger(symbol, SYMBOL_SPREAD);
      info.min_volume = SymbolInfoDouble(symbol, SYMBOL_VOLUME_MIN);

      return info;  // 返回结构体实例
     }

   // sl 止损 tp 止盈
   static bool       buy(string symbol, double volume, double sl=0, double tp=0, string comment = "", long magic=666, double deviation = 20)
     {
      MqlTradeRequest request = {};
      MqlTradeResult result = {};

      double point = SymbolInfoDouble(symbol, SYMBOL_POINT);

      request.action = TRADE_ACTION_DEAL;       // 市价交易
      request.symbol = symbol;                 // 货币对
      request.volume = volume;                 // 交易量
      request.price = SymbolInfoDouble(symbol, SYMBOL_ASK);  // 购买价格
      request.deviation = deviation;           // 最大偏差
      request.magic = magic;                   // 魔术编号
      request.type = ORDER_TYPE_BUY;           // 交易类型：买入
      request.comment = comment;               // 注释

      if(sl != 0)
        {
         request.sl = SymbolInfoDouble(symbol, SYMBOL_BID) - sl * point;   // 设置止损
        }

      if(tp != 0)
        {
         request.tp = SymbolInfoDouble(symbol, SYMBOL_BID) + tp * point;   // 设置止盈
        }

      if(!OrderSend(request, result))
        {
         Print("Buy order failed: ", GetLastError());
         return false;
        }

      Print("Buy order placed successfully!");
      return true;
     }

   static bool       sell(string symbol, double volume, double sl=0, double tp=0, string comment = "", long magic=666, double deviation = 20)
     {
      MqlTradeRequest request = {};
      MqlTradeResult result = {};

      double point = SymbolInfoDouble(symbol, SYMBOL_POINT);

      request.action = TRADE_ACTION_DEAL;       // 市价交易
      request.symbol = symbol;                 // 货币对
      request.volume = volume;                 // 交易量
      request.price = SymbolInfoDouble(symbol, SYMBOL_BID);  // 卖出价格
      request.deviation = deviation;           // 最大偏差
      request.magic = magic;                   // 魔术编号
      request.type = ORDER_TYPE_SELL;          // 交易类型：卖出
      request.comment = comment;               // 注释

      if(sl != 0)
        {
         request.sl = SymbolInfoDouble(symbol, SYMBOL_ASK) + sl * point;   // 设置止损
        }

      if(tp != 0)
        {
         request.tp = SymbolInfoDouble(symbol, SYMBOL_ASK) - tp * point;   // 设置止盈
        }

      if(!OrderSend(request, result))
        {
         Print("Sell order failed: ", GetLastError());
         return false;
        }

      Print("Sell order placed successfully!");
      return true;
     }

   static bool       CloseAllBuy(string symbol, double deviation = 20)
     {
      int total = PositionsTotal();
      for(int i=total-1;i>=0;i--)
        {
         ulong ticket = PositionGetTicket(i);
         if(ticket > 0)
           {
            if(PositionGetString(POSITION_SYMBOL) == symbol &&
               PositionGetInteger(POSITION_TYPE) == POSITION_TYPE_BUY
              )
              {
               MqlTradeRequest request = {};
               MqlTradeResult result = {};
               request.action = TRADE_ACTION_DEAL;
               request.position = ticket;
               request.type = ORDER_TYPE_SELL;
               request.volume = PositionGetDouble(POSITION_VOLUME);
               request.deviation = deviation;
               request.symbol = symbol;
               // 发送交易请求
               if(!OrderSend(request, result))
                 {
                  Print("Error closing order with ticket: ", ticket, ", Error: ", GetLastError());
                  return false;
                 }
              }
           }
        }
      return true;
     }

   static bool       CloseAllSell(string symbol, double deviation = 20)
     {
      int total = PositionsTotal();
      for(int i=total-1;i>=0;i--)
        {
         ulong ticket = PositionGetTicket(i);
         if(ticket > 0)
           {
            if(PositionGetString(POSITION_SYMBOL) == symbol &&
               PositionGetInteger(POSITION_TYPE) == POSITION_TYPE_SELL
              )
              {
               MqlTradeRequest request = {};
               MqlTradeResult result = {};
               request.action = TRADE_ACTION_DEAL;
               request.position = ticket;
               request.type = ORDER_TYPE_BUY;
               request.volume = PositionGetDouble(POSITION_VOLUME);
               request.deviation = deviation;
               request.symbol = symbol;
               // 发送交易请求
               if(!OrderSend(request, result))
                 {
                  Print("Error closing order with ticket: ", ticket, ", Error: ", GetLastError());
                  return false;
                 }
              }
           }
        }
      return true;
     }

   static bool       CloseAll(string symbol, double deviation = 20)
     {
      bool v1 = EasyUtils::CloseAllBuy(symbol,deviation);
      if(v1 == false)
        {
         return false;
        }
      return EasyUtils::CloseAllSell(symbol,deviation);
     }

   static bool       CloseOrderByTicket(int ticket, double deviation = 20)
     {

      int total = PositionsTotal();
      for(int i=total-1;i>=0;i--)
        {
         ulong ticket = PositionGetTicket(i);
         if(ticket > 0)
           {
            if(PositionGetTicket(i) == ticket)
              {
               // 定义请求和结果结构体
               MqlTradeRequest request = {};
               MqlTradeResult result = {};

               // 确定订单类型
               if(PositionGetInteger(POSITION_TYPE) == ORDER_TYPE_BUY)
                 {
                  request.action = TRADE_ACTION_DEAL;
                  request.symbol = PositionGetString(POSITION_SYMBOL);
                  request.volume = PositionGetDouble(POSITION_VOLUME);

                  request.deviation = deviation;
                  request.type = ORDER_TYPE_SELL; // 卖出平仓
                  request.position = ticket;
                 }
               else
                  if(PositionGetInteger(POSITION_TYPE) == ORDER_TYPE_SELL)
                    {
                     request.action = TRADE_ACTION_DEAL;
                     request.symbol = PositionGetString(POSITION_SYMBOL);
                     request.volume = PositionGetDouble(POSITION_VOLUME);

                     request.deviation = deviation;
                     request.type = ORDER_TYPE_BUY; // 买入平仓
                     request.position = ticket;
                    }
                  else
                    {
                     Print("Invalid order type for ticket: ", ticket);
                     return false;
                    }

               // 发送交易请求
               if(!OrderSend(request, result))
                 {
                  Print("Error closing order with ticket: ", ticket, ", Error: ", GetLastError());
                  return false;
                 }

               Print("Order closed successfully, ticket: ", ticket);
               return true;
              }
           }
        }
      return true;

     }
  };

//+------------------------------------------------------------------+
//|                                                                  |
//+------------------------------------------------------------------+
class PositionItem
  {
public:
   string            symbol; // 交易货币对
   double            volume; // 成交量
   double            price_open; // 开仓价格
   ENUM_POSITION_TYPE direction; // 方向
   long              time_msc; // 以毫秒计持仓时间
   ulong ticket; // ticket
  };

// Symbol 信息类定义
class SymbolInfo
{
public:
   double ima[];  // 用于存储 iMA 指标数据的数组
   PositionItem last_position; // 最近的一个订单

   // 构造函数
   SymbolInfo()
   {

   }
   ~SymbolInfo() {
      ArrayFree(ima);
   }

   // 获取 iMA 指标数据的方法
   void ma(
      string symbol,            // 交易品种名称
      ENUM_TIMEFRAMES period,   // 周期
      int ma_period,            // 平均周期
      int ma_shift,             // 平移
      ENUM_MA_METHOD ma_method, // 平滑类型
      ENUM_APPLIED_PRICE applied_price // 价格或者处理程序类型
   )
   {
      int handle;

      // 创建 iMA 指标的句柄
      handle = iMA(symbol, period, ma_period, ma_shift, ma_method, applied_price);

      // 检查句柄是否有效
      if(handle < 0)
      {
         Print("Error creating iMA handle: ", GetLastError());
         return;
      }

      // 从 iMA 指标缓冲区复制数据
      int copied = CopyBuffer(handle, 0, 0, 30, ima);
      if(copied <= 0)
      {
         Print("Error copying iMA buffer: ", GetLastError(), ", copied: ", copied);
         IndicatorRelease(handle);  // 释放句柄
         return;
      }

      ArraySetAsSeries(ima, true);  // 将数组设置为时间序列
      IndicatorRelease(handle);     // 释放句柄
   }

   bool GetLastPosition()
   {
      int total = PositionsTotal();
      if(total > 0)
      {
         ulong ticket = PositionGetTicket(total-1);
         if(ticket > 0)
         {
            last_position.ticket = PositionGetTicket(total-1);
            last_position.symbol = PositionGetString(POSITION_SYMBOL);
            last_position.volume = PositionGetDouble(POSITION_VOLUME);
            last_position.price_open = PositionGetDouble(POSITION_PRICE_OPEN);
            last_position.direction = PositionGetInteger(POSITION_TYPE);
            last_position.time_msc = PositionGetInteger(POSITION_TIME_MSC); // 以毫秒计持仓时间
            return true;
         }
      }
      return false;
   }
};

//+------------------------------------------------------------------+

long MinutesToMilliseconds(int minutes) {
    return minutes * 60 * 1000;  // 1 分钟 = 60 * 1000 毫秒
}
