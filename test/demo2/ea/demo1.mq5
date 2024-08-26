//+------------------------------------------------------------------+
//|                                                        demo1.mq5 |
//|                                  Copyright 2024, MetaQuotes Ltd. |
//|                                             https://www.mql5.com |
//+------------------------------------------------------------------+
#property copyright "Copyright 2024, MetaQuotes Ltd."
#property link      "https://www.mql5.com"
#property version   "1.00"

enum duokong
{
  buy,   // BUY
  sell,  // SELL
};

input int limit = 10;  // 输入框
input duokong dk; // 方向
input ENUM_TIMEFRAMES time; // 时间周期
//+------------------------------------------------------------------+
//| Expert initialization function                                   |
//+------------------------------------------------------------------+

void demo1();
int OnInit()
  {
//--- create timer
   EventSetTimer(60);  // 设置定时器
    double list_a[];  // 定义数组
    Print("Initial size: ", ArraySize(list_a));  // 打印初始数组大小
    ArrayResize(list_a, 1);  // 动态的list 必须 调整数组大小
    list_a[0] = 1.01;
    ArrayResize(list_a, 10);  // 调整数组大小
    list_a[1] = 1.02;
    list_a[2] = 1.03;
    Print("Resized size: ", ArraySize(list_a));  // 打印调整后的数组大小
    demo1();
//---
   return(INIT_SUCCEEDED);
  }
//+------------------------------------------------------------------+
//| Expert deinitialization function                                 |
//+------------------------------------------------------------------+
void OnDeinit(const int reason)  // 当关闭ea时执行
  {
//--- destroy timer
   EventKillTimer();   // 关闭定时器

  }
//+------------------------------------------------------------------+
//| Expert tick function                                             |
//+------------------------------------------------------------------+
void OnTick()  // 当tick变动时执行
  {
//---

  }
//+------------------------------------------------------------------+
//| Timer function                                                   |
//+------------------------------------------------------------------+
void OnTimer()  // 定时器60s执行一次
  {
//---

  }
//+------------------------------------------------------------------+
//| Trade function                                                   |
//+------------------------------------------------------------------+
void OnTrade()  // 当发生交易活动时执行
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
//| BookEvent function                                               |
//+------------------------------------------------------------------+
void OnBookEvent(const string &symbol) // 当市场深度变化时执行
  {
//---

  }
//+------------------------------------------------------------------+

double a_list[]; // 可变数组
double b_list[10]; // 固定数组


class User
{
public:  // 类的成员和方法应使用public关键字公开
   string name;
   int age;

   // 构造函数
   User(string name, int age) {
      this.name = name;
      this.age = age;
   }

   // 增加年龄的方法
   void next_year() {
      this.age++;
   }

   // 打印用户信息的方法
   void print_info() {
      Print("Name: ", this.name, ", Age: ", this.age);
   }
};

void demo1() {
   // 创建一个User对象
   User user("zhangsan", 14);

   // 输出初始信息
   user.print_info();

   // 调用next_year方法
   user.next_year();

   // 输出更新后的信息
   user.print_info();
}