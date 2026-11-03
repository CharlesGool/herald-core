# herald-core:产物是 GSI 补丁 dist/herald-core/(拷进 TB321FU-GSI-Mods/patches/ 即可用)。
# patch/  补丁源码(broker、heraldctl、init、sepolicy、patch.conf)
# engine/ 认证引擎源码(fork 自 OhMyKeymint,标识已改 herald),在 ubt-dev 编出 keymint / inject,放进 attestation-fix 的 payload
#
#   make dist     补丁产物 dist/herald-core/
#   make check    脚本语法 + 借 Mods 仓库的 herald 回归测试跑本仓库的 patch/(scripts/check.sh)
#   make engine   在 ubt-dev 上编 keymint / inject(engine/scripts/build-android.sh),产物和 BUILD-INFO 拷回 dist/engine/
ENGINE_REMOTE := /root/build/herald-engine
.PHONY: dist check engine clean
dist:
	rm -rf dist/herald-core && mkdir -p dist/herald-core && cp -r patch/. dist/herald-core/
	@echo "产物: dist/herald-core/"
check:
	scripts/check.sh
# 远端先清掉上次的源码(留 target 作缓存),删掉的文件才不会残留;BUILD-INFO 里记下源码提交号
engine:
	ssh ubt-dev 'mkdir -p $(ENGINE_REMOTE) && find $(ENGINE_REMOTE) -mindepth 1 -maxdepth 1 ! -name target -exec rm -rf {} +'
	tar --exclude=target --exclude=dist -cf - engine | ssh ubt-dev 'tar -xf - -C $(ENGINE_REMOTE) --strip-components=1'
	ssh ubt-dev 'HERALD_SOURCE_COMMIT=$(shell git rev-parse --short HEAD)$(shell git diff --quiet HEAD -- engine || echo -dirty) bash $(ENGINE_REMOTE)/scripts/build-android.sh'
	mkdir -p dist/engine && scp ubt-dev:$(ENGINE_REMOTE)/target/aarch64-linux-android/release/keymint ubt-dev:$(ENGINE_REMOTE)/target/aarch64-linux-android/release/inject ubt-dev:$(ENGINE_REMOTE)/BUILD-INFO dist/engine/
	@echo "产物: dist/engine/(keymint、inject、BUILD-INFO)"
clean:
	rm -rf dist
