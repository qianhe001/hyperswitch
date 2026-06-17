import sys
sys.path.append('../')
from core.request import Request
import hashlib
from datetime import datetime

#   
#   接收汇付交易结果异步回调通知示例
#  

from flask import Flask, request, jsonify

app = Flask(__name__)
class ReceiveHuifuNotifyController(Request):
	@app.route('/callback', methods=['POST'])
	def callback():
		# 回调签名 sign
		sign = request.headers.get('X-Signature')
		
		# 回调数据 data
		data = request.data

		# 处理数据
		print(f'X-Signature: {sign}')
		print(f'Received data: {data}')
		string = data + Request.config['secretKey']       
		if (sign == hashlib.md5(string.encode('utf-8')).hexdigest()):
			print(f'验签成功')
		else:
			print(f'验签失败')

		# 返回响应
		# return jsonify({'status': 'success', 'x_sign': sign, 'data': data})

if __name__ == '__main__':
    app.run(port=5000)