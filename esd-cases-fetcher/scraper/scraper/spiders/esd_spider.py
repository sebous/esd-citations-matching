import scrapy
from urllib import parse


class EsdSpider(scrapy.Spider):
    name = "esd"
    # must be cz version
    start_urls = [
        "https://curia.europa.eu/juris/documents.jsf?oqp=&for=&mat=or&lgrec=cs&jge=&td=%24mode%3DfromTo%24from%3D1952.01.01%24to%3D2018.12.31%3B%3BNPUB%3BPUB1%2CPUB2%2CPUB7%3BNPUB1%2CNPUB2%2CNPUB3%3B%3B%3BORDALL&jur=C&page=1&dates=&pcs=Oor&lg=&pro=&nat=or&cit=none%252CC%252CCJ%252CR%252C2008E%252C%252C%252C%252C%252C%252C%252C%252C%252C%252Ctrue%252Cfalse%252Cfalse&language=cs&avg=&cid=5479422"
    ]

    def parse(self, response):
        query_params = parse.parse_qs(parse.urlsplit(response.url).query)
        curr_page = int(query_params["page"][0])

        for row in response.css("table.detail_table_documents tr.table_document_ligne"):
            yield {
                "code": row.css(".table_cell_aff::text").get().strip(),
                "date": row.css(".table_cell_date::text").get().strip(),
                "short_name": row.css(".table_cell_nom_usuel::text").get().strip(),
                "metadata": {
                    "a": 1,
                    "b": 2,
                }
            }

        last_page = int(response.css("div.pagination").re(r'(\d+) strany')[0])
        if curr_page >= last_page:
            return

        next_page_url = response.css(
            ".pagination > a::attr(href)").getall()[-2]
        yield response.follow(next_page_url, self.parse)
