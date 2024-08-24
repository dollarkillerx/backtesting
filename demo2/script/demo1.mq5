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

#define change "常量"  // 不能使用 ;

void OnStart()
  {
//---
   // 基础数据类型
   Alert(change);
   int a = 12;
   float b = 12.0;
   double c = 12.00;
   string d = "hello world";

   // 复合数据类型
   color f=clrBlue;
   datetime e=D'2015.01.01 00:00';

   int array[] = {1, 2, 3, 4};  // 定义一个整数数组
   struct MyStruct {
    int x;
    float y;
    string z;
   };

   MyStruct example = {1, 2.0, "example"};

   int list[];  // 动态数组可以作为列表使用
   ArrayResize(list, 3);  // 调整数组大小
   list[0] = 10;
   list[1] = 20;
   list[2] = 30;


   struct KeyValue {
       string key;
       int value;
   };

   KeyValue map[];
   ArrayResize(map, 2);
   map[0].key = "first";
   map[0].value = 100;
   map[1].key = "second";
   map[1].value = 200;

   Alert(add(12,213));

   int j1 = StringToInteger("123");
   printf("j1: %d\n",j1);
  }
//+------------------------------------------------------------------+
int add(int a, int b) {
   return a + b;
}