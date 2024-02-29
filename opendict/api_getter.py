import requests
import os
import json

# https://opendict.korean.go.kr/service/openApiInfo
url = "https://opendict.korean.go.kr/api/search"
with open("api_key.txt", "r", encoding="utf-8") as f:
    key = f.read().strip()
param = {
    "key": key,
    "q": "가",
    "req_type": "json",
    "start": 1,  # 1 ~ 1000
    "num": 100,  # 1 ~ 100
    "method": "include",
    "advanced": "y",
    "pos": 0,  # 품사
    "region": 0,  # 방언 지역
    "cat": 0,  # 전문 분야
}

print(
    """
- 품사
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
)
param["pos"] = input("품사를 고르세요 : ").replace(" ", "")

print(
    """
- 방언 지역
0: 전체
- 아래 값을 다중 선택할 수 있도록 콤마(,)로 구분하여 나열한다.
1: 강원
2: 경기
3: 경남
4: 경북
5: 경상
6: 전남
7: 전라
8: 전북
9: 제주
10: 충남
11: 충북
12: 충청
13: 평남
14: 평북
15: 평안
16: 함경
17: 함남
18: 함북
19: 황해
20: 중국 길림성
21: 중국 요령성
22: 중국 흑룡강성
23: 중앙아시아
"""
)
param["region"] = input("방언 지역을 고르세요 : ").replace(" ", "")

print(
    """
- 전문 분야
0: 전체
- 아래 값을 다중 선택할 수 있도록 콤마(,)로 구분하여 나열한다.
1: 가톨릭
2: 건설
3: 경영
4: 경제
5: 고유명 일반
6: 공업
7: 공예
8: 공학 일반
9: 광업
10: 교육
11: 교통
12: 군사
13: 기계
14: 기독교
15: 농업
16: 동물
17: 매체
18: 무용
19: 문학
20: 물리
21: 미술
22: 민속
23: 법률
24: 보건 일반
25: 복식
26: 복지
27: 불교
28: 사회 일반
29: 산업 일반
30: 생명
31: 서비스업
32: 수산업
33: 수의
34: 수학
35: 식물
36: 식품
37: 심리
38: 약학
39: 언어
40: 역사
41: 연기
42: 영상
43: 예체능 일반
44: 음악
45: 의학
46: 인명
47: 인문 일반
48: 임업
49: 자연 일반
50: 재료
51: 전기·전자
52: 정보·통신
53: 정치
54: 종교 일반
55: 지구
56: 지리
57: 지명
58: 책명
59: 천문
60: 천연자원
61: 철학
62: 체육
63: 한의
64: 해양
65: 행정
66: 화학
67: 환경
"""
)
param["cat"] = input("전문 분야를 고르세요 : ").replace(" ", "")

if not os.path.exists("result"):
    os.mkdir("result")

for start in range(1, 1001):
    param["start"] = start
    result = requests.get(url, params=param)
    filepath = (
        "result/"
        + str(param["q"])
        + "_pos"
        + str(param["pos"])
        + "_region"
        + str(param["region"])
        + "_cat"
        + str(param["cat"])
        + "_page"
        + str(param["start"])
        + ".json"
    )
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(result.text)
    if len(json.loads(result.text)["channel"]["item"]) < param["num"]:
        break
