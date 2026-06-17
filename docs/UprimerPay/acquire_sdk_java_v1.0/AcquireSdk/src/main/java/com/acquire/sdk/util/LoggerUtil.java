package com.acquire.sdk.util;

import java.io.File;
import java.io.IOException;
import java.text.SimpleDateFormat;
import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.Date;
import java.util.logging.*;

public class LoggerUtil {
    private static final Logger logger = Logger.getLogger(LoggerUtil.class.getName()); 

    static {
        try {
            // 设置日志级别
            logger.setLevel(Level.ALL);

            // 创建FileHandler，指定日志文件路径
            File file = new File(LoggerUtil.class.getResource("").getPath()); 
            String currentDir = file.getAbsolutePath();
            Date date = new Date();
            SimpleDateFormat sdf = new SimpleDateFormat("yyyy-MM-dd");
            String today = sdf.format(date);
            String filepath =  currentDir + "/../log/" + today + ".log" ;
            FileHandler fileHandler = new FileHandler(filepath, true);
   
            // 设置日志文件的格式
            LocalDateTime now = LocalDateTime.now();
            DateTimeFormatter formatter = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
            String formattedDateTime = now.format(formatter);
            fileHandler.setFormatter(new SimpleFormatter() {
                @Override
                public synchronized String format(LogRecord record) {
                    // String date = new Date().toString();
                    // return date + " " + record.getLevel() + ": " + record.getMessage() + "\n";                
                    return formattedDateTime + " " + record.getLevel() + ": " + record.getMessage() + "\n";
                }
            });

            // 添加日志处理器
            logger.addHandler(fileHandler);
        } catch (IOException e) {
            // 如果配置失败，打印错误信息到标准错误输出
            System.err.println("Error setting up file handler: " + e.getMessage());
        }
    }

    // 静态方法，用于记录日志
    public static void log(Level level, String message) {
        logger.log(level, message);
    }

    // 可以添加更多方法，比如info, warning, severe等
    public static void info(String message) {
        log(Level.INFO, message);
    }

    public static void warning(String message) {
        log(Level.WARNING, message);
    }

    public static void severe(String message) {
        log(Level.SEVERE, message);
    }

    // ... 其他日志级别方法
}