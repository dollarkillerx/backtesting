``` 
   if(last_position.last_position.direction == POSITION_TYPE_BUY)
     {
      easyUtils.Buy(symbol, new_volume);
     }
   else
     {
      easyUtils.Sell(symbol, new_volume);
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
     
     
   int total = PositionsTotal();
   if(total > 0)
     {
      for(int i=total-1;i>=0;i--)
        {
            new_volume = new_volume * multiple;
         }
     }

    new_volume = MathFloor(new_volume * 100) / 100;
```