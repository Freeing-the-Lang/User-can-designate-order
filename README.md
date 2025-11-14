# 🧩 User-can-designate-order
### MultiMain-OS / Spongelang — User Defined Execution Order Module  
여러 개의 main() 또는 MeaningNode가 존재할 때,  
기본 자동 실행 그래프 대신 **사용자가 직접 순서를 지정할 수 있는 기능**을 제공하는 모듈입니다.

이 리포는 MultiMain-OS의 “옵션 정책 레이어”이며,  
Spongelang Core 또는 OS 본체에 포함되지 않습니다.

---

## 🎯 목적
MultiMain-OS는 기본적으로 **의미 기반 자동 실행 흐름(Meaning Graph)** 으로 동작합니다.  
그러나 특정 상황에서는 유저가 직접 실행 순서를 지정하고 싶을 수 있습니다.

이 모듈은 다음을 지원합니다:

- 유저가 지정한 순서를 우선 적용  
- 여러 main() 우선순위 지정  
- MeaningNode 간 수동 흐름 제어  
- 자동 그래프와 수동 그래프를 병합 또는 교체 가능  
- `.mmo`(MultiMain Order) 포맷 제공

---

## 🧪 사용 예시

### 📁 프로젝트 구조



project/
├── main.rs
├── main.go
├── hello.mml
├── analysis.swift
└── order.mmo   ← 사용자 지정 순서 파일



### ✏️ order.mmo 예시



main.go
analysis.swift
hello.mml
main.rs



- 첫 번째로 Go main 실행  
- 다음 Swift 의미 노드  
- 그 다음 Spongelang 파일  
- 마지막으로 Rust main

자동 그래프와 다르더라도 **강제 순서**로 실행된다.

---

## ▶ 실행 방법



mmain . --order order.mmo



또는




mmain --order ./order.mmo project/



MultiMain-OS는 내부적으로:

1. Spongelang Meaning Graph 생성  
2. order.mmo를 읽어 수동 우선순위 적용  
3. 의미 DAG 재구성  
4. 최종 실행

---

## 📦 `.mmo` 파일 포맷
간단한 한 줄–한 노드 구조:




파일명 또는 MeaningNodeID
파일명 또는 MeaningNodeID
...



예:



init.mml
graphics.swift
ai.rs
print.go



추가 확장 기능:

### 🟦 그룹 지정



[group setup]
init.mml
config.mml



### 🟦 DAG 강제 edge



analysis.swift -> report.mml



### 🟦 무시 목록



ignore: debug.rs
ignore: test.swift



---

## 🔌 MultiMain-OS 연동 구조
User-can-designate-order는 OS 본체에 직접 포함되지 않고  
**독립 모듈**로 동작합니다.




Spongelang Core
↓
Meaning Runtime
↓
MultiMain-OS
↓
───────────────
User-can-designate-order   ← (현재 리포)
───────────────



---

## 🧱 설치 (Coming soon)
이 모듈은 MultiMain-OS 빌드 시 자동 포함되거나  
독립 플러그인 형태로 로드됩니다.

예시:



mmain plugin add user-can-designate-order



---

## 🚫 지원하지 않는 것
- C 언어(main.c) 순서 지정 (전체 생태계 C 제외 정책 유지)
- C/C++ ABI 기반 노드
- 빌드 단계 순서 제어 (런타임 순서만 제어)

---

## 🔮 철학
- 자동 의미 그래프가 기본  
- 그러나 사용자가 개입할 수 있는 작은 여지를 제공  
- Spongelang의 “의미 우선” 철학을 해치지 않음  
- MultiMain-OS의 유연성 확장

> “자동이 기본이지만, 필요할 때는 유저가 조정할 수 있어야 한다.”

---

## 📄 라이선스
MIT License




