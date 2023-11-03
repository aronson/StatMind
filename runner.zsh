#!/usr/bin/env zsh
codesign --entitlements entitlements.xml -fs 44EB90D60071EB66D745AF5BFE4AF92EE72D6389 "$1"
exec "$1"