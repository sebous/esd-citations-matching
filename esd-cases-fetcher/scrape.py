import os

target_path = os.path.join(os.path.dirname(__file__), "./scraper")

stream = os.popen(f'cd {target_path} && scrapy crawl esd -O data.json')
print(stream.read())
