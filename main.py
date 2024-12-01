import binascii
import nfc

def on_connect(tag):
    # システムコードを指定（0x0003は交通系ICカードの共通システムコード）
    sc = 0x0003
    tag.sys = sc

    # サービスコード（履歴情報取得用）
    service_code = 0x090f

    # サービスコードのリストを作成
    service_codes = [nfc.tag.tt3.ServiceCode(service_code >> 6, service_code & 0x3f)]

    # ブロック番号のリストを作成（0から19までの20件の履歴を取得）
    blocks = [nfc.tag.tt3.BlockNumber(i, service=0) for i in range(20)]

    # ブロックデータの読み取り
    try:
        data = tag.read_without_encryption(service_codes, blocks)
        for i in range(20):
            block = data[i*16:(i+1)*16]
            print(f"ブロック{i}: {binascii.hexlify(block)}")
    except Exception as e:
        print(f"読み取りエラー: {e}")

# リーダーに接続
clf = nfc.ContactlessFrontend('usb')
clf.connect(rdwr={'on-connect': on_connect})
