# CCML 1.0 Project Charter

## 1. Goal
- CCML을 MVP 수준에서 프로덕션 배포 가능한 `v1.0.0`으로 재구축한다.
- 핵심 원칙:
  - Convenient for humans
  - Compatible with JSON (strict)
  - Clear to parsers

## 2. Inputs / Outputs
- Inputs:
  - `CONTRIBUTING.md`의 언어 원칙
  - `old_temp/`의 MVP 구현 및 테스트
- Outputs:
  - CCML 1.0 draft spec
  - 안정적인 parser/library/CLI
  - conformance + regression + property tests
  - CI/CD 및 릴리즈 체계

## 3. Scope
- In scope (1.0):
  - JSON 호환 파싱
  - 주석, 암시적 루트 객체, 선택적 콤마
  - 라인/컬럼 포함 에러 진단
  - Python 라이브러리 API와 CLI
- Out of scope (1.0):
  - include/import
  - 스키마 언어
  - JSON 바깥의 신규 타입

## 4. Hyper-Waterfall Workflow

### Phase 1: Plan
- 목적/범위/리스크 정의
- 산출물:
  - 본 헌장 문서
  - 초기 리스크 레지스터
- Exit criteria:
  - JSON strict compatibility가 명시되고 합의됨

### Phase 2: Design
- 서비스 경계, 디렉터리 구조, 의존성 정책 정의
- 산출물:
  - `docs/architecture.md`
- Exit criteria:
  - 단방향 의존성
  - 순환 의존성 0

### Phase 3: Implement
- 작은 단위로 구현하고 즉시 검증
- 구현 순서:
  - spec 기반 core parser
  - semantic validation/diagnostics
  - CLI/tooling
- Exit criteria:
  - 공개 API/CLI 동작이 문서와 일치

### Phase 4: Validate
- 기능/엣지케이스/성능/회귀 검증
- 산출물:
  - 테스트 리포트
  - 호환성 결과
- Exit criteria:
  - conformance 100%
  - 회귀 0
  - 정의된 SLO 충족

### Phase 5: Document
- 사용자/개발자/운영 문서 완성
- 산출물:
  - spec + user/developer docs
  - 릴리즈 노트 템플릿
- Exit criteria:
  - 신규 사용자가 문서만으로 설치/실행 가능

## 5. Quality Gates (Release Blockers)
- JSON compatibility 테스트 실패
- 파싱 오류에서 라인/컬럼 미제공
- 명세와 구현 불일치
- 테스트 없는 public behavior 변경

## 6. Milestone Draft (5 weeks)
1. Week 1 (completed): Plan/Design freeze + core parser baseline + conformance runner prototype.
2. Week 2: FFI phase-1 implementation + FFI contract tests + Python/Node binding kickoff.
3. Week 3: Binding stabilization (Python first, Node skeleton) + conformance expansion.
4. Week 4: QA hardening + CI pipeline integration + RC candidate.
5. Week 5: Release preparation + `v1.0.0` release.
