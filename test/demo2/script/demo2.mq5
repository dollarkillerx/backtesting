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
   long account;            // 账户
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
   double point;            // 返回当前交易品种大小点    一般这个为最小交易量
   long sperad;             // 点差
   double min_volume;       // 最低持仓数量
   // double step;             // 步长
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
      info.sperad = SymbolInfoInteger(Symbol(),SYMBOL_SPREAD);
      info.min_volume = SymbolInfoDouble(Symbol(),SYMBOL_VOLUME_MIN);
      // info.step = SymbolInfoDouble(Symbol(),SYMBOL_VOLUME_STEP);

      return info;  // 返回结构体实例
   }
};

// 示例使用
void OnStart() {
   AccountInfoPayload accountInfo = EasyUtils::GetAccountInfo();
   Print("Account Name: ", accountInfo.account_name);
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

      Print("ACCOUNT_LOGIN: ", SymbolInfoDouble(Symbol(),SYMBOL_VOLUME_MIN));


   int symbolTotal = SymbolsTotal(true); // true只显示添加到市场报价窗口的所有交易品种
   Print("totoal: ", symbolTotal);
   Print("totoal: ", SymbolsTotal(false));   // false显示全部
   // SymbolName();
   // SymbolSelect();

   int total = SymbolsTotal(false);
   for(int i=0; i<total; i++)
     {
      //--- 通过循环索引获取列表中交易品种的名称
      string name = SymbolName(i, false);

      //--- 如果这是所需的交易品种，则将其名称和在列表中的位置发送到日志并退出循环
      if(name == "LTCUSDm")
        {
         PrintFormat("The '%s' symbol was found in the list of server symbols at index %d", name, i);
         Print("bid: ", SymbolInfoDouble(name, SYMBOL_BID));
         break;
        }
     }

     // SymbolIsSynchronized() // 数据是否为同步的
}
