# 2025-04-24

- [x] create dockerfile to run project
- [x] refine dockerfile with cargo chef for build speedup  
- [x] update readme
- [ ] create welcome endpoint and profile endpoint in grpc and http
- [ ] require authenication for profile endpoint
- [ ] update config with required field for oauth flow ( generic but with zitadel in mind )
- [ ] research how create implementation for endpoints that will not require to implement twice same endpoints for grpc and http clients
- [ ] create implementation of endpoint that can handle both grpc and http without redefining implementation
- [ ] create proto grpc definition for oauth 
- [ ] create http endpoints for oauth
- [ ] create mocked implementation for oauth flow
- [ ] create integration tests for oauth flow (mocked implementation)
- [ ] create unit tests for endpoints to test authentication requirements
- [ ] create new crate that will handle all of database / storage for server application. use sqlx

```kroki-mermaid
graph TD
  A[ Anyone ] -->|Can help | B( Go to github.com/yuzutech/kroki )
  B --> C{ How to contribute? }
  C --> D[ Reporting bugs ]
  C --> E[ Sharing ideas ]
  C --> F[ Advocating ]
```