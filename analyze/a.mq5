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
