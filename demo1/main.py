import vectorbt as vbt
import yfinance as yf
import os
import pandas as pd

CACHE_FILE = 'eurusd_price.csv'

def get_price_data():
    if os.path.exists(CACHE_FILE):
        # 从缓存文件加载数据
        print("Loading data from cache...")
        price = pd.read_csv(CACHE_FILE, index_col=0, parse_dates=True)
    else:
        # 下载数据并保存到缓存文件
        print("Downloading data...")
        price = vbt.YFData.download('EURUSD=X', period="5d", interval='1m').get('Close')
        price.to_csv(CACHE_FILE)
    return price


if __name__ == '__main__':
    price = get_price_data()
    # print(price)

    # 计算 RSI 指标
    rsi = vbt.RSI.run(price, window=14)
    print(rsi)