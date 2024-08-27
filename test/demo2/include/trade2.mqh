//+------------------------------------------------------------------+
//|                                                        trade.mqh |
//|                                  Copyright 2024, MetaQuotes Ltd. |
//|                                 https://github.com/dollarkillerx |
//+------------------------------------------------------------------+
#property copyright "Copyright 2024, MetaQuotes Ltd."
#property link      "https://github.com/dollarkillerx"
#include <Trade\Trade.mqh>

// 账户基本信息
struct AccountInfoPayload {
   string account_name;     // 账户名称
   long account;            // 账户登录ID
   string account_server;   // 交易服务器
   string account_company;  // 提供账户公司名称
   bool ea_ok;              // EA 是否可用

   double balance;          // 余额
   double profit;           // 利润
   double margin;           // 使用保证金
   long level;              // 杠杆
};

// Symbol货币基础信息
struct SymbolInfoPayload {
   string symbol;           // 当前图表货币名称
   ENUM_TIMEFRAMES period;  // 当前时间表
   int digits;              // 当前货币对交易小数点
   double point;            // 返回当前交易品种大小点（最小交易量）
   long spread;             // 点差
   double min_volume;       // 最低持仓数量
};

// 工具类定义
class EasyUtils {
public:
   CTrade m_trade;  // Trading object

   EasyUtils(string symbol, int magic, ulong deviation) {
      m_trade.SetExpertMagicNumber(magic);
      m_trade.SetMarginMode();
      m_trade.SetTypeFillingBySymbol(symbol);
      m_trade.SetDeviationInPoints(deviation);
   }

   // 获取账户信息的方法
   AccountInfoPayload GetAccountInfo() {
      AccountInfoPayload info;

      info.account_name = AccountInfoString(ACCOUNT_NAME);
      info.account = AccountInfoInteger(ACCOUNT_LOGIN);
      info.account_server = AccountInfoString(ACCOUNT_SERVER);
      info.account_company = AccountInfoString(ACCOUNT_COMPANY);

      info.balance = AccountInfoDouble(ACCOUNT_BALANCE);
      info.profit = AccountInfoDouble(ACCOUNT_PROFIT);
      info.margin = AccountInfoDouble(ACCOUNT_MARGIN);
      info.level = AccountInfoInteger(ACCOUNT_LEVERAGE);

      info.ea_ok = (AccountInfoInteger(ACCOUNT_TRADE_EXPERT) == 1);

      return info;
   }

   // 获取货币符号信息的方法
   SymbolInfoPayload GetSymbolInfo(string symbol) {
      SymbolInfoPayload info;

      info.symbol = symbol;
      info.period = Period();
      info.digits = SymbolInfoInteger(symbol, SYMBOL_DIGITS);
      info.point = SymbolInfoDouble(symbol, SYMBOL_POINT);
      info.spread = SymbolInfoInteger(symbol, SYMBOL_SPREAD);
      info.min_volume = SymbolInfoDouble(symbol, SYMBOL_VOLUME_MIN);

      return info;
   }

   // sl 止损 tp 止盈
   bool Buy(string symbol, double volume, double sl=0, double tp=0, string comment="") {
      double point = SymbolInfoDouble(symbol, SYMBOL_POINT);
      double price = SymbolInfoDouble(symbol, SYMBOL_BID);

      sl = (sl != 0) ? price - sl * point : 0;
      tp = (tp != 0) ? price + tp * point : 0;

      if (!m_trade.Buy(volume, symbol, 0, sl, tp, comment)) {
         Print("Buy order failed: ", GetLastError());
         return false;
      }

      Print("Buy order placed successfully!");
      return true;
   }

   bool Sell(string symbol, double volume, double sl=0, double tp=0, string comment="") {
      double point = SymbolInfoDouble(symbol, SYMBOL_POINT);
      double price = SymbolInfoDouble(symbol, SYMBOL_ASK);

      sl = (sl != 0) ? price + sl * point : 0;
      tp = (tp != 0) ? price - tp * point : 0;

      if (!m_trade.Sell(volume, symbol, 0, sl, tp, comment)) {
         Print("Sell order failed: ", GetLastError());
         return false;
      }

      Print("Sell order placed successfully!");
      return true;
   }

   bool CloseAll(string symbol) {
      while (PositionsTotal() != 0) {
         if (!m_trade.PositionClose(symbol)) {
            Print("Failed to close position: ", GetLastError());
            return false;
         }
      }
      return true;
   }

   bool CloseOrderByTicket(int ticket) {
      return m_trade.PositionClose(ticket);
   }
};

//+------------------------------------------------------------------+

class PositionItem {
public:
   string symbol;  // 交易货币对
   double volume;  // 成交量
   double price_open;  // 开仓价格
   ENUM_POSITION_TYPE direction;  // 方向
   long time_msc;  // 以毫秒计持仓时间
   ulong ticket;  // ticket
};

// Symbol 信息类定义
class SymbolInfo {
public:
   double ima[];  // 用于存储 iMA 指标数据的数组
   PositionItem last_position;  // 最近的一个订单

   // 构造函数
   SymbolInfo() {}

   // 析构函数
   ~SymbolInfo() {
      ArrayFree(ima);
   }

   // 获取 iMA 指标数据的方法
   void ma(string symbol, ENUM_TIMEFRAMES period, int ma_period, int ma_shift, ENUM_MA_METHOD ma_method, ENUM_APPLIED_PRICE applied_price) {
      int handle = iMA(symbol, period, ma_period, ma_shift, ma_method, applied_price);

      if (handle < 0) {
         Print("Error creating iMA handle: ", GetLastError());
         return;
      }

      int copied = CopyBuffer(handle, 0, 0, 30, ima);
      if (copied <= 0) {
         Print("Error copying iMA buffer: ", GetLastError(), ", copied: ", copied);
      } else {
         ArraySetAsSeries(ima, true);
      }

      IndicatorRelease(handle);  // 释放句柄
   }

   bool GetLastPosition() {
      int total = PositionsTotal();
      if (total > 0) {
         ulong ticket = PositionGetTicket(total - 1);
         if (ticket > 0) {
            last_position.ticket = ticket;
            last_position.symbol = PositionGetString(POSITION_SYMBOL);
            last_position.volume = PositionGetDouble(POSITION_VOLUME);
            last_position.price_open = PositionGetDouble(POSITION_PRICE_OPEN);
            last_position.direction = PositionGetInteger(POSITION_TYPE);
            last_position.time_msc = PositionGetInteger(POSITION_TIME_MSC);
            return true;
         }
      }
      return false;
   }
};

//+------------------------------------------------------------------+

// 工具函数
long MinutesToMilliseconds(int minutes) {
   return (long)minutes * 60 * 1000;
}
