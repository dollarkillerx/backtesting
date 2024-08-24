// 结构体定义
struct AccountInfoPayload {
   string account_name;     // 账户名称
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
   double point;            // 返回当前交易品种大小点
};

// 工具类定义
class EasyUtils {

public:
   // 获取账户信息的方法
   static AccountInfoPayload GetAccountInfo() {
      AccountInfoPayload info;  // 创建结构体实例

      // 设置结构体的各个属性值
      info.account_name = AccountInfoString(ACCOUNT_NAME);
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

      return info;  // 返回结构体实例
   }
};
