import os

target_path = os.path.join(os.path.dirname(__file__), "./scraper")

stream = os.popen(
    f'cd {target_path} && scrapy crawl esd --set FEED_EXPORT_ENCODING=utf-8 -O data.csv:csv')
print(stream.read())
