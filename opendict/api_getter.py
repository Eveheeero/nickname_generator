import requests

# https://opendict.korean.go.kr/service/openApiInfo
url = "https://opendict.korean.go.kr/api/search"
param = {
    "key": "KEY",
    "q": "안녕",
    "req_type": "json",
    "start": 1,
    "num": 100,
    "advanced": "y",
    "pos": 1,  # 품사
}
"""
- 품사(기본값 0)
0: 전체
- 아래 값을 다중 선택할 수 있도록 콤마(,)로 구분하여 나열한다.
1: 명사
2: 대명사
3: 수사
4: 조사
5: 동사
6: 형용사
7: 관형사
8: 부사
9: 감탄사
10: 접사
11: 의존 명사
12: 보조 동사
13: 보조 형용사
14: 어미
15: 관형사·명사
16: 수사·관형사
17: 명사·부사
18: 감탄사·명사
19: 대명사·부사
20: 대명사·감탄사
21: 동사·형용사
22: 관형사·감탄사
23: 부사·감탄사
24: 의존명사·조사
25: 수사·관형사·명사
26: 대명사·관형사
27: 품사 없음
"""
response = requests.get(url, param)

with open("api_response.json", "w", encoding="utf-8") as f:
    f.write(response.text)
