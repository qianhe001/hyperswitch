import sys
sys.path.append('../')
import logging
from logging.handlers import TimedRotatingFileHandler
import datetime

logging.basicConfig(
    level=logging.INFO,  # 日志级别为INFO
    format='%(asctime)s %(levelname)s %(message)s',  # 日志格式
    datefmt='%Y-%m-%d %H:%M:%S',  # 日期格式
    encoding='utf-8'
)

# 将日志输出到文件中，并启用日志轮转
logFile = '../log/' + str(datetime.date.today()) + '.log'
handler = TimedRotatingFileHandler(logFile, when='D', interval=1, backupCount=7, encoding='utf-8')
handler.setLevel(logging.INFO)
handler.setFormatter(logging.Formatter('%(asctime)s %(levelname)s %(message)s'))
logging.getLogger().addHandler(handler)
