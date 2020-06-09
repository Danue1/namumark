# namumark.rs

namumark.rs는 나무위키의 마크업 파싱 및 렌더링을 지원하는 라이브러리입니다.

- 개발 중인 라이브러리입니다.

# 참여

namumark.rs는 여러분의 기여 및 참여를 환영합니다.

버그, 부족한 기능, 미비한 문법이 있다면 Issues를 통해 제보해 주시기 바랍니다.

# 로드맵

- [x] 기본 문법 파싱
- [ ] 문서 파싱
- [ ] JSON/XML 렌더러
- [ ] 기본 HTML 렌더러
- [ ] 메타데이터 추출 기능
- [ ] 편의성 커맨드라인 툴
- [ ] WASM 컴파일
- [ ] etc

# 파싱 지원

- [x] 일반 텍스트
- [x] 리다이렉트(#redirect)
- [x] 문단

  - [x] 열린 문단

    - [x] h1
    - [x] h2
    - [x] h3
    - [x] h4
    - [x] h5
    - [x] h6

  - [x] 닫힌 문단

    - [x] h1
    - [x] h2
    - [x] h3
    - [x] h4
    - [x] h5
    - [x] h6

- [x] 들여쓰기(indent)
- [x] 인용(blockquote)
- [x] 수평줄(hr)
- [x] 시맨틱

  - [x] 취소선(delete)
  - [x] 기울임(emphasis)
  - [x] 굵기(strong)
  - [x] 아래첨자(subscript)
  - [x] 위첨자(superscript)
  - [x] 밑줄(underline)

- [x] 리스트

  - [x] 번호 없는 리스트

    - [x] 문법 무효화 문법을 통한 여러 줄 작성

  - [x] 번호 있는 리스트

    - [x] 문법 무효화 문법을 통한 여러 줄 작성
    - [x] 시작 번호 지정
    - [x] 숫자
    - [x] 한글 초성/음절
    - [x] 알파벳 대/소문자
    - [x] 아라비안 대/소문자

- [ ] 테이블

  - [ ] 넓이 지정
  - [ ] 높이 지정
  - [ ] 정렬 기준 지정
  - [ ] 가로 합치기
  - [ ] 세로 합치기
  - [ ] 배경색 지정

    - [ ] 테이블
    - [ ] 로우
    - [ ] 컬럼
    - [ ] 배경

  - [ ] 글자색 지정

    - [ ] 테이블
    - [ ] 로우
    - [ ] 컬럼
    - [ ] 글자

  - [ ] 테두리색 지정
  - [ ] 추가 파라미터 확인 필요

- [x] bracket 문법(`{{{text}}}`)

  - [x] 글자 크기

    - [x] -1
    - [x] -2
    - [x] -3
    - [x] -4
    - [x] -5
    - [x] +1
    - [x] +2
    - [x] +3
    - [x] +4
    - [x] +5

  - [x] 글자색 지정
  - [x] 접기
  - [x] 문법 강조(syntax highlight)
  - [x] 문법 무효화
  - [ ] HTML 직접 사용
  - [ ] 줄바꿈 기준 지정

- [ ] 괄호 2개 문법(command)

  - [ ] 분류

    - [x] 기본 파싱
    - [ ] 링크 처리(`[[:분류:]]`)

  - [ ] 링크

    - [x] 기본 파싱
    - [ ] 출력이 같은 링크
    - [ ] 출력이 다른 링크
    - [ ] 특정 문단 링크
    - [ ] 상위 문서 링크
    - [ ] 하위 문서 링크
    - [ ] 외부 페이지 링크

  - [ ] 이미지

    - [x] 넓이 지정
    - [x] 높이 지정
    - [x] 정렬 기준 지정
    - [x] 배경색 지정
    - [ ] 추가 파라미터 확인 필요

  - [ ] 비디오

    - [x] Youtube

      - [x] 넓이 지정
      - [x] 높이 지정
      - [x] 시작점 지정
      - [x] 끝점 지정

    - [x] KakaoTV

      - [x] 넓이 지정
      - [x] 높이 지정
      - [x] 시작점 지정
      - [x] 끝점 지정

    - [x] NicoVideo

      - [x] 넓이 지정
      - [x] 높이 지정
      - [x] 시작점 지정
      - [x] 끝점 지정

    - [ ] HTML5를 통한 비디오 임베드
    - [ ] 추가 파라미터 확인 필요

- [ ] 괄호 1개 문법(macro)

  - [x] 나이(age)
  - [x] 앵커(anchor)
  - [x] 주석(comment)

    - [x] 라벨 지정

  - [x] 날짜(date, datetime)
  - [x] D-Day(dday)
  - [x] 각주(footnote)
  - [ ] 틀(include)

    - [x] 기본 파싱
    - [ ] 추가 파라미터 확인 필요

  - [x] 수식(math)
  - [x] 줄바꿈(br)
  - [x] 페이지카운트(pagecount)
  - [x] 루비(ruby)
  - [x] 목차(tableofcontents)

# 렌더링 지원

WIP
