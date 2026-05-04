# Weekly Plan 2 (2026-05-05 to 2026-05-11)

## 1. Weekly Goal
1. `ccml-ffi` phase-1 C ABI 구현 완료.
2. FFI 계약 테스트 및 conformance smoke 경로 확보.
3. Python/Node 바인딩 착수 가능한 경계 확정.

## 2. Scope
- In scope:
  - JSON-only FFI surface (`to_json`, `diagnose`, `version`, `free`)
  - return-code + error-json 계약
  - alloc/free 메모리 계약
  - FFI 경유 conformance smoke 검증
  - Python thin adapter 초안
- Out of scope:
  - Node 완전 바인딩 구현
  - 배포 자동화 완성
  - v1.0.0 릴리즈 작업

## 3. Day-by-Day Plan

### Day 1 - Tue 2026-05-05
- Freeze FFI phase-1 interface:
  - 함수 시그니처
  - 리턴코드 표준
  - 메모리 해제 계약
- Deliverables:
  - FFI interface draft 문서
  - `ccml-ffi` 기본 스켈레톤

### Day 2 - Wed 2026-05-06
- Implement `ccml_to_json` + `ccml_diagnose`.
- Implement argument validation (null/UTF-8).
- Deliverables:
  - 핵심 FFI 함수 동작
  - 오류 JSON 반환 경로

### Day 3 - Thu 2026-05-07
- Implement `ccml_version` + `ccml_free`.
- Add panic guard and internal error mapping.
- Deliverables:
  - 메모리 계약 완성
  - 안전성 경계 처리

### Day 4 - Fri 2026-05-08
- Add FFI contract tests:
  - success/error/warn path
  - pointer/argument safety
- Deliverables:
  - FFI 테스트 세트
  - 실패 케이스 재현/검증 로그

### Day 5 - Sat 2026-05-09
- Conformance smoke via FFI path:
  - representative `valid/invalid/warn` fixtures
- Deliverables:
  - FFI 경유 검증 리포트
  - parity gap 목록

### Day 6 - Sun 2026-05-10
- Python thin adapter kickoff.
- Minimal usage tests for:
  - transcode
  - diagnose
  - duplicate warning propagation
- Deliverables:
  - Python adapter 초안
  - smoke test 결과

### Day 7 - Mon 2026-05-11
- Weekly validation and sprint-close:
  - pass-rate snapshot
  - known gaps and blockers
  - Week 3 priorities
- Deliverables:
  - week2 review summary
  - sprint3 backlog draft

## 4. Definition of Done (Week 2)
1. `ccml-ffi` phase-1 함수 세트 구현 완료.
2. FFI contract tests 통과.
3. FFI 경유 conformance smoke 통과.
4. Python adapter로 핵심 호출 성공.
5. Week 3 착수를 위한 갭 목록 문서화 완료.

## 5. Risks and Mitigations
1. Risk: FFI 메모리 해제 규약 누락/오용.
- Mitigation: 모든 반환 포인터를 `ccml_free` 경로로 통일하고 테스트로 강제.
2. Risk: 바인딩별 예외 처리 편차.
- Mitigation: return-code + error-json 계약을 바인딩 공통 규약으로 고정.
3. Risk: conformance parity 누락.
- Mitigation: Week 2 내 FFI 경유 smoke를 필수 게이트로 설정.

## 6. Priority Order
1. ABI 안정성
2. 진단/오류 계약 일관성
3. 바인딩 사용성
