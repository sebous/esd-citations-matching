import scrapy


class EsdSpider(scrapy.Spider):
    name = "esd"
    start_urls = [
        "https://curia.europa.eu/juris/documents.jsf?oqp=&for=&mat=or&jge=&td=%24mode%3D1Y%24from%3D2020.12.19%24to%3D2021.12.19%3B%3B%3BPUB1%2CPUB2%3B%3B%3B%3BORDALL&jur=C%2CT%2CF&page=1&dates=&pcs=Oor&lg=&pro=&nat=or&cit=none%252CC%252CCJ%252CR%252C2008E%252C%252C%252C%252C%252C%252C%252C%252C%252C%252Ctrue%252Cfalse%252Cfalse&language=cs&avg=&cid=1676655"
    ]

    def parse(self, response):
        for row in response.css("table.detail_table_documents tr.table_document_ligne"):
            yield {
                "code": row.css(".table_cell_aff::text").get().strip(),
                "date": row.css(".table_cell_date::text").get().strip(),
                "short_name": row.css(".table_cell_nom_usuel::text").get().strip()
            }

        # TODO: detect when on last page, stop crawling
        next_page_url = response.css(
            ".pagination > a::attr(href)").getall()[-2]
        yield response.follow(next_page_url, self.parse)
