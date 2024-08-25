//+------------------------------------------------------------------+
//|                                                        demo1.mq5 |
//|                                  Copyright 2024, MetaQuotes Ltd. |
//|                                             https://www.mql5.com |
//+------------------------------------------------------------------+
#property copyright "Copyright 2024, MetaQuotes Ltd."
#property link      "https://www.mql5.com"
#property version   "1.00"
//+------------------------------------------------------------------+
//| Script program start function                                    |
//+------------------------------------------------------------------+



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
   // 获取账户信息的方法
   static AccountInfoPayload GetAccountInfo() {
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
   static SymbolInfoPayload GetSymbolInfo() {
      SymbolInfoPayload info;

      info.symbol = Symbol();
      info.period = Period();
      info.digits = Digits();
      info.point = Point();
      info.spread = SymbolInfoInteger(Symbol(), SYMBOL_SPREAD);
      info.min_volume = SymbolInfoDouble(Symbol(), SYMBOL_VOLUME_MIN);

      return info;  // 返回结构体实例
   }
};

// Symbol 信息类定义
class SymbolInfo {
public:
   double ima[];  // 用于存储 iMA 指标数据的数组

   // 构造函数
   SymbolInfo() {
      ArrayResize(ima, 30);  // 调整数组大小为 30
   }

   // 获取 iMA 指标数据的方法
   void ma(
      string symbol,            // 交易品种名称
      ENUM_TIMEFRAMES period,   // 周期
      int ma_period,            // 平均周期
      int ma_shift,             // 平移
      ENUM_MA_METHOD ma_method, // 平滑类型
      ENUM_APPLIED_PRICE applied_price // 价格或者处理程序类型
   ) {
      int handle;

      // 创建 iMA 指标的句柄
      handle = iMA(symbol, period, ma_period, ma_shift, ma_method, applied_price);

      // 检查句柄是否有效
      if(handle < 0) {
         Print("Error creating iMA handle: ", GetLastError());
         return;
      }

      // 从 iMA 指标缓冲区复制数据
      if(CopyBuffer(handle, 0, 0, 30, ima) <= 0) {
         Print("Error copying iMA buffer: ", GetLastError());
         IndicatorRelease(handle);  // 释放句柄
         return;
      }

      ArraySetAsSeries(ima, true);  // 将数组设置为时间序列
      IndicatorRelease(handle);     // 释放句柄
   }
};

// 示例使用
void OnStart() {
   AccountInfoPayload accountInfo = EasyUtils::GetAccountInfo();
   Print("Account Name: ", accountInfo.account_name);
   Print("Account ID: ", accountInfo.account);
   Print("Account Server: ", accountInfo.account_server);
   Print("Account Company: ", accountInfo.account_company);
   Print("Balance: ", accountInfo.balance);
   Print("Profit: ", accountInfo.profit);
   Print("Margin: ", accountInfo.margin);
   Print("Leverage: ", accountInfo.level);
   Print("EA OK: ", accountInfo.ea_ok);

   SymbolInfoPayload symbolInfo = EasyUtils::GetSymbolInfo();
   Print("Symbol: ", symbolInfo.symbol);
   Print("Period: ", symbolInfo.period);
   Print("Digits: ", symbolInfo.digits);
   Print("Point: ", symbolInfo.point);
   Print("Spread: ", symbolInfo.spread);
   Print("Min Volume: ", symbolInfo.min_volume);

   SymbolInfo info;

   info.ma("LTCUSDm",PERIOD_H1 , 20, 0, MODE_SMA, PRICE_CLOSE);
   Print("First MA Value: ", info.ima[0]);
}
