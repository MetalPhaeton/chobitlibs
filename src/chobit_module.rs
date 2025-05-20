// Copyright (C) 2022 Hironori Ishibashi
//
// This work is free. You can redistribute it and/or modify it under the
// terms of the Do What The Fuck You Want To Public License, Version 2,
// as published by Sam Hocevar. See below for more details.
//
// --------------------------------------------------------------------
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

#![allow(dead_code)]

//! WASM module library.
//!
//! This library can be used to create WebAssembly modules in the Actor Model style.
//!
//! If you can use ReScript, the following libraries are also useful.  
//! (Usage is written in `*.resi` files.)
//!
//! - <a href="data:text/plain;base64,Ly8gQ29weXJpZ2h0IChDKSAyMDI1IEhpcm9ub3JpIElzaGliYXNoaQovLwovLyBUaGlzIHdvcmsgaXMgZnJlZS4gWW91IGNhbiByZWRpc3RyaWJ1dGUgaXQgYW5kL29yIG1vZGlmeSBpdCB1bmRlciB0aGUKLy8gdGVybXMgb2YgdGhlIERvIFdoYXQgVGhlIEZ1Y2sgWW91IFdhbnQgVG8gUHVibGljIExpY2Vuc2UsIFZlcnNpb24gMiwKLy8gYXMgcHVibGlzaGVkIGJ5IFNhbSBIb2NldmFyLiBTZWUgYmVsb3cgZm9yIG1vcmUgZGV0YWlscy4KLy8KLy8gLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgICAgICAgICAgICAgICAgICBWZXJzaW9uIDIsIERlY2VtYmVyIDIwMDQKLy8KLy8gQ29weXJpZ2h0IChDKSAyMDA0IFNhbSBIb2NldmFyIDxzYW1AaG9jZXZhci5uZXQ+Ci8vCi8vIEV2ZXJ5b25lIGlzIHBlcm1pdHRlZCB0byBjb3B5IGFuZCBkaXN0cmlidXRlIHZlcmJhdGltIG9yIG1vZGlmaWVkCi8vIGNvcGllcyBvZiB0aGlzIGxpY2Vuc2UgZG9jdW1lbnQsIGFuZCBjaGFuZ2luZyBpdCBpcyBhbGxvd2VkIGFzIGxvbmcKLy8gYXMgdGhlIG5hbWUgaXMgY2hhbmdlZC4KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgVEVSTVMgQU5EIENPTkRJVElPTlMgRk9SIENPUFlJTkcsIERJU1RSSUJVVElPTiBBTkQgTU9ESUZJQ0FUSU9OCi8vCi8vICAwLiBZb3UganVzdCBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPLgoKdHlwZSB3b3JrZXIKdHlwZSBtZXNzYWdlID0gKGJpZ2ludCwgYmlnaW50LCBVaW50OEFycmF5LnQpCnR5cGUgZXZlbnQgPSB7CiAgZGF0YTogbWVzc2FnZQp9CkBuZXcgZXh0ZXJuYWwgY3JlYXRlV29ya2VyOiAoc3RyaW5nLCB7InR5cGUiOiBzdHJpbmd9KSA9PiB3b3JrZXIgPSAiV29ya2VyIgpAc2VuZCBleHRlcm5hbCBwb3N0TWVzc2FnZTogKAogIHdvcmtlciwKICBtZXNzYWdlLAogIGFycmF5PEFycmF5QnVmZmVyLnQ+CikgPT4gdW5pdCA9ICJwb3N0TWVzc2FnZSIKQHNlbmQgZXh0ZXJuYWwgdGVybWluYXRlOiAod29ya2VyLCB1bml0KSA9PiB1bml0ID0gInRlcm1pbmF0ZSIKQHNldCBleHRlcm5hbCBvbk1lc3NhZ2U6ICh3b3JrZXIsIChldmVudCkgPT4gdW5pdCkgPT4gdW5pdCA9ICJvbm1lc3NhZ2UiCgovLyBzeXN0ZW0gZGF0YSAtLS0tLS0tLS0tCmxldCByZWdpc3RyeTogTWFwLnQ8YmlnaW50LCB3b3JrZXI+ID0gTWFwLm1ha2UoKQpsZXQgc2VsZklkOiBiaWdpbnQgPSAwbgovLyAtLS0tLS0tLS0tCgpsZXQgdGVsbDogKGJpZ2ludCwgYmlnaW50LCBVaW50OEFycmF5LnQpID0+IHVuaXQgPQogIChzZW5kZXJJZCwgcmVjZWl2ZXJJZCwgZGF0YSkgPT4gewogICAgc3dpdGNoIHJlZ2lzdHJ5LT5NYXAuZ2V0KHJlY2VpdmVySWQpIHsKICAgICAgfCBOb25lID0+ICgpCiAgICAgIHwgU29tZShyZWNlaXZlcikgPT4gewogICAgICAgIHJlY2VpdmVyLT5wb3N0TWVzc2FnZSgKICAgICAgICAgIChzZW5kZXJJZCwgcmVjZWl2ZXJJZCwgZGF0YSksCiAgICAgICAgICBbZGF0YS0+VHlwZWRBcnJheS5idWZmZXJdCiAgICAgICAgKQogICAgICB9CiAgICB9CiAgfQoKbGV0IGFkZEFjdG9yOiAoc3RyaW5nLCBiaWdpbnQsIChiaWdpbnQsIFVpbnQ4QXJyYXkudCkgPT4gdW5pdCkgPT4gdW5pdCA9CiAgKGFjdG9ySnNVcmwsIGlkLCBvbk1lc3NhZ2VIYW5kbGVyKSA9PiB7CiAgICBzd2l0Y2ggcmVnaXN0cnktPk1hcC5nZXQoaWQpIHsKICAgICAgfCBOb25lID0+ICgpCiAgICAgIHwgU29tZSh3b3JrZXIpID0+IHsKICAgICAgICBDb25zb2xlLndhcm4oCiAgICAgICAgICAie1wid2FyblwiOlwiQWN0b3JBbHJlYWR5RXhpc3RzXCIsXCJpZFwiOiIKICAgICAgICAgICAgKysgaWQtPkJpZ0ludC50b1N0cmluZwogICAgICAgICAgICArKyAifSIKICAgICAgICApCiAgICAgICAgd29ya2VyLT50ZXJtaW5hdGUoKQogICAgICB9CiAgICB9CgogICAgbGV0IGFjdG9ySnNVcmwgPSBhY3RvckpzVXJsICsrICI/aWQ9IiArKyBpZC0+QmlnSW50LnRvU3RyaW5nCgogICAgbGV0IHdvcmtlciA9IGNyZWF0ZVdvcmtlcihhY3RvckpzVXJsLCB7InR5cGUiOiAibW9kdWxlIn0pCgogICAgd29ya2VyLT5vbk1lc3NhZ2UoKGV2ZW50KSA9PiB7CiAgICAgIGxldCAoc2VuZGVySWQsIHJlY2VpdmVySWQsIHBheWxvYWQpID0gZXZlbnQuZGF0YQoKICAgICAgaWYgcmVjZWl2ZXJJZCA9PT0gc2VsZklkIHsKICAgICAgICBvbk1lc3NhZ2VIYW5kbGVyKHNlbmRlcklkLCBwYXlsb2FkKQogICAgICB9IGVsc2UgewogICAgICAgIHRlbGwoc2VuZGVySWQsIHJlY2VpdmVySWQsIHBheWxvYWQpCiAgICAgIH0KICAgIH0pCgogICAgcmVnaXN0cnktPk1hcC5zZXQoaWQsIHdvcmtlcikKICB9CgpsZXQgZGVsZXRlQWN0b3I6IChiaWdpbnQpID0+IGJvb2wgPSAoaWQpID0+IHsKICBzd2l0Y2ggcmVnaXN0cnktPk1hcC5nZXQoaWQpIHsKICAgIHwgTm9uZSA9PiBDb25zb2xlLndhcm4oCiAgICAgICJ7XCJ3YXJuXCI6XCJBY3Rvck5vdEZvdW5kXCIsXCJpZFwiOiIKICAgICAgICArKyBpZC0+QmlnSW50LnRvU3RyaW5nCiAgICAgICAgKysgIn0iCiAgICApCiAgICB8IFNvbWUod29ya2VyKSA9PiB7CiAgICAgIHdvcmtlci0+dGVybWluYXRlKCkKICAgIH0KICB9CgogIHJlZ2lzdHJ5LT5NYXAuZGVsZXRlKGlkKQp9CgpsZXQgc2VuZE1lc3NhZ2U6IChiaWdpbnQsIFVpbnQ4QXJyYXkudCkgPT4gdW5pdCA9CiAgKHJlY2VpdmVySWQsIGRhdGEpID0+IHRlbGwoc2VsZklkLCByZWNlaXZlcklkLCBkYXRhKQogIApsZXQgY291bnRBY3RvcnM6IHVuaXQgPT4gaW50ID0gKCkgPT4gcmVnaXN0cnktPk1hcC5zaXplCgpsZXQgaGFzQWN0b3I6IGJpZ2ludCA9PiBib29sID0gKGlkKSA9PiByZWdpc3RyeS0+TWFwLmhhcyhpZCkK" download=ChobitModuleSystem.res>ChobitModuleSystem.res</a>
//! - <a href="data:text/plain;base64,Ly8gQ29weXJpZ2h0IChDKSAyMDI1IEhpcm9ub3JpIElzaGliYXNoaQovLwovLyBUaGlzIHdvcmsgaXMgZnJlZS4gWW91IGNhbiByZWRpc3RyaWJ1dGUgaXQgYW5kL29yIG1vZGlmeSBpdCB1bmRlciB0aGUKLy8gdGVybXMgb2YgdGhlIERvIFdoYXQgVGhlIEZ1Y2sgWW91IFdhbnQgVG8gUHVibGljIExpY2Vuc2UsIFZlcnNpb24gMiwKLy8gYXMgcHVibGlzaGVkIGJ5IFNhbSBIb2NldmFyLiBTZWUgYmVsb3cgZm9yIG1vcmUgZGV0YWlscy4KLy8KLy8gLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgICAgICAgICAgICAgICAgICBWZXJzaW9uIDIsIERlY2VtYmVyIDIwMDQKLy8KLy8gQ29weXJpZ2h0IChDKSAyMDA0IFNhbSBIb2NldmFyIDxzYW1AaG9jZXZhci5uZXQ+Ci8vCi8vIEV2ZXJ5b25lIGlzIHBlcm1pdHRlZCB0byBjb3B5IGFuZCBkaXN0cmlidXRlIHZlcmJhdGltIG9yIG1vZGlmaWVkCi8vIGNvcGllcyBvZiB0aGlzIGxpY2Vuc2UgZG9jdW1lbnQsIGFuZCBjaGFuZ2luZyBpdCBpcyBhbGxvd2VkIGFzIGxvbmcKLy8gYXMgdGhlIG5hbWUgaXMgY2hhbmdlZC4KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgVEVSTVMgQU5EIENPTkRJVElPTlMgRk9SIENPUFlJTkcsIERJU1RSSUJVVElPTiBBTkQgTU9ESUZJQ0FUSU9OCi8vCi8vICAwLiBZb3UganVzdCBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPLgoKLy8gQWRkcyBuZXcgY2hvYml0IG1vZHVsZSBhY3RvciwgYW5kIHJ1bnMgb24gd29ya2VyIHRocmVhZC4gIAovLyBUaGlzIG1haW4gdGhyZWFkIElEIGlzIDAsIHNvIGRvbid0IHVzZSAwIGZvciBhY3RvciBJRC4gIAovLyBJZiBhbiBhY3RvciB0aGF0IHNhbWUgSUQgYWxyZWFkeSBleGlzdHMsIHRoZSBvbGQgYWN0b3IgaXMgdGVybWluYXRlZAovLyBhbmQgcmVnaXN0b3IgbmV3IGFjdG9yLgovLwovLyAtIDFzdCBhcmc6IFBhdGggdG8gYWN0b3Igc2NyaXB0LgovLyAtIDJuZCBhcmc6IEFjdG9yIElELgovLyAtIDNyZCBhcmc6IEhhbmRsZXIgdGhhdCB0aGUgbWFpbiB0aHJlYWQgcmVjZWl2ZWQgYSBtZXNzYWdlIGZyb20gdGhlIGFjdG9yLgovLyAgICAgLSAxc3QgYXJnOiBUaGUgYWN0b3IgSUQuCi8vICAgICAtIDJuZCBhcmc6IE1lc3NhZ2UgZnJvbSB0aGUgYWN0b3IuCmxldCBhZGRBY3RvcjogKHN0cmluZywgYmlnaW50LCAoYmlnaW50LCBVaW50OEFycmF5LnQpID0+IHVuaXQpID0+IHVuaXQKCi8vIFRlcm1pbmF0ZXMgYW5kIGRlbGV0ZXMgY2hvYml0IG1vZHVsZSBhY3Rvci4KLy8KLy8gLSAxc3QgYXJnOiBBY3RvciBJRCB0aGF0IGlzIGRlbGV0ZWQuCi8vIC0gUmV0dXJuOiBJZiB0aGUgYWN0b3IgZXhpc3RzLCByZXR1cm5zIHRydWUuCmxldCBkZWxldGVBY3RvcjogYmlnaW50ID0+IGJvb2wKCi8vIFNlbmRzIG1lc3NhZ2UgdG8gYW4gYWN0b3IuICAKLy8gKFRoZSBtZXNzYWdlIGJ1ZmZlciBpcyB0cmFuc2ZlcmVkIHRvIHRoZSBhY3Rvciwgc28gYWZ0ZXIgc2VuZGluZywgdGhlIG1lc3NhZ2UgYmVjYW1lcyBlbXB0eSBvbiB0aGlzIG1haW4gdGhyZWFkLikKLy8KLy8gLSAxc3QgYXJnOiBBY3RvciBJRC4KLy8gLSAybmQgYXJnOiBNZXNzYWdlLgpsZXQgc2VuZE1lc3NhZ2U6IChiaWdpbnQsIFVpbnQ4QXJyYXkudCkgPT4gdW5pdAoKLy8gQ291bnRzIGNob2JpdCBtb2R1bGUgYWN0b3JzCi8vCi8vIC0gUmV0dXJuOiBOdW1iZXIgb2YgYWN0b3JzLgpsZXQgY291bnRBY3RvcnM6IHVuaXQgPT4gaW50CgovLyBHZXRzIHdoZXRoZXIgdGhlIGFjdG9yIGV4aXN0cyBvciBub3QuCi8vCi8vIC0gMXN0IGFyZzogQWN0b3IgSUQuCmxldCBoYXNBY3RvcjogYmlnaW50ID0+IGJvb2wK" download=ChobitModuleSystem.resi>ChobitModuleSystem.resi</a>
//! - <a href="data:text/plain;base64,Ly8gQ29weXJpZ2h0IChDKSAyMDI1IEhpcm9ub3JpIElzaGliYXNoaQovLwovLyBUaGlzIHdvcmsgaXMgZnJlZS4gWW91IGNhbiByZWRpc3RyaWJ1dGUgaXQgYW5kL29yIG1vZGlmeSBpdCB1bmRlciB0aGUKLy8gdGVybXMgb2YgdGhlIERvIFdoYXQgVGhlIEZ1Y2sgWW91IFdhbnQgVG8gUHVibGljIExpY2Vuc2UsIFZlcnNpb24gMiwKLy8gYXMgcHVibGlzaGVkIGJ5IFNhbSBIb2NldmFyLiBTZWUgYmVsb3cgZm9yIG1vcmUgZGV0YWlscy4KLy8KLy8gLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgICAgICAgICAgICAgICAgICBWZXJzaW9uIDIsIERlY2VtYmVyIDIwMDQKLy8KLy8gQ29weXJpZ2h0IChDKSAyMDA0IFNhbSBIb2NldmFyIDxzYW1AaG9jZXZhci5uZXQ+Ci8vCi8vIEV2ZXJ5b25lIGlzIHBlcm1pdHRlZCB0byBjb3B5IGFuZCBkaXN0cmlidXRlIHZlcmJhdGltIG9yIG1vZGlmaWVkCi8vIGNvcGllcyBvZiB0aGlzIGxpY2Vuc2UgZG9jdW1lbnQsIGFuZCBjaGFuZ2luZyBpdCBpcyBhbGxvd2VkIGFzIGxvbmcKLy8gYXMgdGhlIG5hbWUgaXMgY2hhbmdlZC4KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgVEVSTVMgQU5EIENPTkRJVElPTlMgRk9SIENPUFlJTkcsIERJU1RSSUJVVElPTiBBTkQgTU9ESUZJQ0FUSU9OCi8vCi8vICAwLiBZb3UganVzdCBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPLgoKQHZhbCBleHRlcm5hbCBnbG9iYWxUaGlzOiBEb20ud2luZG93ID0gImdsb2JhbFRoaXMiCkBnZXQgZXh0ZXJuYWwgbG9jYXRpb246IERvbS53aW5kb3cgPT4gRG9tLmxvY2F0aW9uID0gImxvY2F0aW9uIgpAZ2V0IGV4dGVybmFsIHNlYXJjaDogRG9tLmxvY2F0aW9uID0+IHN0cmluZyA9ICJzZWFyY2giCgp0eXBlIHVybFNlYXJjaFBhcmFtcwpAbmV3IGV4dGVybmFsIGNyZWF0ZVVybFNlYXJjaFBhcmFtczogc3RyaW5nID0+IHVybFNlYXJjaFBhcmFtcyA9CiAgIlVSTFNlYXJjaFBhcmFtcyIKQHNlbmQgZXh0ZXJuYWwgZ2V0VmFsdWU6ICh1cmxTZWFyY2hQYXJhbXMsIHN0cmluZykgPT4gc3RyaW5nID0gImdldCIKQHZhbCBleHRlcm5hbCBzdHJpbmdUb0JpZ2ludDogc3RyaW5nID0+IGJpZ2ludCA9ICJCaWdJbnQiCgpsZXQgc2VhcmNoUGFyYW1zID0gY3JlYXRlVXJsU2VhcmNoUGFyYW1zKGdsb2JhbFRoaXMtPmxvY2F0aW9uLT5zZWFyY2gpCmxldCBzZWxmSWQgPSBzdHJpbmdUb0JpZ2ludChzZWFyY2hQYXJhbXMtPmdldFZhbHVlKCJpZCIpKQoKdHlwZSBpbXBvcnRGdW5jcyA9IHsKICAgIG5vdGlmeV9pbnB1dF9idWZmZXI6IChpbnQsIGludCkgPT4gdW5pdCwKICAgIG5vdGlmeV9vdXRwdXRfYnVmZmVyOiAoaW50LCBpbnQpID0+IHVuaXQsCiAgICBzZW5kOiAoYmlnaW50LCBpbnQpID0+IHVuaXQKfQp0eXBlIHdhc21JbXBvcnRzID0gewogIGVudjogaW1wb3J0RnVuY3MKfQoKdHlwZSB3YXNtTWVtb3J5ID0gewogIGJ1ZmZlcjogQXJyYXlCdWZmZXIudAp9CnR5cGUgd2FzbUV4cG9ydHMgPSB7CiAgbWVtb3J5OiB3YXNtTWVtb3J5LAogIGluaXQ6IGJpZ2ludCA9PiB1bml0LAogIHJlY3Y6IChiaWdpbnQsIGludCkgPT4gdW5pdAp9CnR5cGUgd2FzbUluc3RhbmNlID0gewogIGV4cG9ydHM6IHdhc21FeHBvcnRzCn0KdHlwZSByZXN1bHRPYmplY3QgPSB7CiAgaW5zdGFuY2U6IHdhc21JbnN0YW5jZQp9Cgp0eXBlIHJlc3BvbnNlCkB2YWwgZXh0ZXJuYWwgZmV0Y2g6IHN0cmluZyA9PiByZXNwb25zZSA9ICJmZXRjaCIKCkBzY29wZSgiV2ViQXNzZW1ibHkiKSBAdmFsIGV4dGVybmFsIGluc3RhbnRpYXRlU3RyZWFtaW5nOgoocmVzcG9uc2UsIHdhc21JbXBvcnRzKSA9PiBwcm9taXNlPHJlc3VsdE9iamVjdD4gPSAiaW5zdGFudGlhdGVTdHJlYW1pbmciCgp0eXBlIG1lc3NhZ2UgPSAoYmlnaW50LCBiaWdpbnQsIFVpbnQ4QXJyYXkudCkKQHNlbmQgZXh0ZXJuYWwgcG9zdE1lc3NhZ2U6IChEb20ud2luZG93LCBtZXNzYWdlKSA9PiB1bml0ID0gInBvc3RNZXNzYWdlIgpAc2VuZCBleHRlcm5hbCBwb3N0TWVzc2FnZTI6ICgKICBEb20ud2luZG93LAogIG1lc3NhZ2UsCiAgYXJyYXk8QXJyYXlCdWZmZXIudD4KKSA9PiB1bml0ID0gInBvc3RNZXNzYWdlIgoKdHlwZSBldmVudCA9IHsKICBkYXRhOiBtZXNzYWdlCn0KQHNldCBleHRlcm5hbCBvbk1lc3NhZ2U6IChEb20ud2luZG93LCAoZXZlbnQpID0+IHVuaXQpID0+IHVuaXQgPSAib25tZXNzYWdlIgoKQHNlbmQgZXh0ZXJuYWwgb3ZlcndyaXRlOiAoVWludDhBcnJheS50LCBVaW50OEFycmF5LnQpID0+IHVuaXQgPSAic2V0IgoKbGV0IGluc3RhbmNlOiByZWY8d2FzbUluc3RhbmNlPiA9IHJlZih7CiAgZXhwb3J0czogewogICAgbWVtb3J5OiB7YnVmZmVyOiBBcnJheUJ1ZmZlci5tYWtlKDApfSwKICAgIGluaXQ6IChfKSA9PiAoKSwKICAgIHJlY3Y6IChfLCBfKSA9PiAoKQogIH0KfSkKCmxldCBpbnB1dEJ1ZmZlciA9IHJlZihVaW50OEFycmF5LmZyb21MZW5ndGgoMCkpCmxldCBvdXRwdXRCdWZmZXIgPSByZWYoVWludDhBcnJheS5mcm9tTGVuZ3RoKDApKQoKbGV0IGxvYWRXYXNtOiBzdHJpbmcgPT4gcHJvbWlzZTx1bml0PiA9ICh1cmwpID0+IHsKICBvcGVuIFR5cGVkQXJyYXkKICBsZXQgaW1wb3J0T2JqOiB3YXNtSW1wb3J0cyA9IHsKICAgIGVudjogewogICAgICBub3RpZnlfaW5wdXRfYnVmZmVyOiAoYWRkcmVzcywgc2l6ZSkgPT4gewogICAgICAgIGxldCBtZW0gPSBVaW50OEFycmF5LmZyb21CdWZmZXIoCiAgICAgICAgICBpbnN0YW5jZS5jb250ZW50cy5leHBvcnRzLm1lbW9yeS5idWZmZXIKICAgICAgICApCgogICAgICAgIGlucHV0QnVmZmVyIDo9IG1lbS0+c3ViYXJyYXkoCiAgICAgICAgICB+c3RhcnQgPSBhZGRyZXNzLAogICAgICAgICAgfmVuZCA9IGFkZHJlc3MgKyBzaXplCiAgICAgICAgKQogICAgICB9LAoKICAgICAgbm90aWZ5X291dHB1dF9idWZmZXI6IChhZGRyZXNzLCBzaXplKSA9PiB7CiAgICAgICAgbGV0IG1lbSA9IFVpbnQ4QXJyYXkuZnJvbUJ1ZmZlcigKICAgICAgICAgIGluc3RhbmNlLmNvbnRlbnRzLmV4cG9ydHMubWVtb3J5LmJ1ZmZlcgogICAgICAgICkKCiAgICAgICAgb3V0cHV0QnVmZmVyIDo9IG1lbS0+c3ViYXJyYXkoCiAgICAgICAgICB+c3RhcnQgPSBhZGRyZXNzLAogICAgICAgICAgfmVuZCA9IGFkZHJlc3MgKyBzaXplCiAgICAgICAgKQogICAgICB9LAoKICAgICAgc2VuZDogKHJlY2VpdmVySWQsIHNpemUpID0+IHsKICAgICAgICBsZXQgcGF5bG9hZCA9IG91dHB1dEJ1ZmZlci5jb250ZW50cy0+c3ViYXJyYXkoCiAgICAgICAgICB+c3RhcnQgPSAwLAogICAgICAgICAgfmVuZCA9IHNpemUKICAgICAgICApCgogICAgICAgIGdsb2JhbFRoaXMtPnBvc3RNZXNzYWdlKChzZWxmSWQsIHJlY2VpdmVySWQsIHBheWxvYWQpKQogICAgICB9CiAgICB9CiAgfQoKICBvcGVuIFByb21pc2UKICBpbnN0YW50aWF0ZVN0cmVhbWluZyhmZXRjaCh1cmwpLCBpbXBvcnRPYmopLT50aGVuUmVzb2x2ZSgocmVzdWx0KSA9PiB7CiAgICBpbnN0YW5jZSA6PSByZXN1bHQuaW5zdGFuY2UKCiAgICBnbG9iYWxUaGlzLT5vbk1lc3NhZ2UoKGV2ZW50KSA9PiB7CiAgICAgIGxldCAoc2VuZGVySWQsIF9yZWNlaXZlcklkLCBwYXlsb2FkKSA9IGV2ZW50LmRhdGEKCiAgICAgIGxldCBsZW4gPSBpZiBpbnB1dEJ1ZmZlci5jb250ZW50cy0+bGVuZ3RoIDwgcGF5bG9hZC0+bGVuZ3RoIHsKICAgICAgICBpbnB1dEJ1ZmZlci5jb250ZW50cy0+bGVuZ3RoCiAgICAgIH0gZWxzZSB7CiAgICAgICAgcGF5bG9hZC0+bGVuZ3RoCiAgICAgIH0KCiAgICAgIGxldCBwYXlsb2FkID0gcGF5bG9hZC0+c3ViYXJyYXkoCiAgICAgICAgfnN0YXJ0ID0gMCwKICAgICAgICB+ZW5kID0gbGVuCiAgICAgICkKCiAgICAgIGlucHV0QnVmZmVyLmNvbnRlbnRzLT5vdmVyd3JpdGUocGF5bG9hZCkKCiAgICAgIGluc3RhbmNlLmNvbnRlbnRzLmV4cG9ydHMucmVjdihzZW5kZXJJZCwgbGVuKQogICAgfSkKCiAgICBpbnN0YW5jZS5jb250ZW50cy5leHBvcnRzLmluaXQoc2VsZklkKQogIH0pCn0KCmxldCBzZW5kTWVzc2FnZTogKGJpZ2ludCwgVWludDhBcnJheS50KSA9PiB1bml0ID0gKHJlY2VpdmVySWQsIGRhdGEpID0+IHsKICBnbG9iYWxUaGlzLT5wb3N0TWVzc2FnZTIoCiAgICAoc2VsZklkLCByZWNlaXZlcklkLCBkYXRhKSwKICAgIFtkYXRhLT5UeXBlZEFycmF5LmJ1ZmZlcl0KICApCn0KCkBzZW5kIGV4dGVybmFsIGNsb3NlOiAoRG9tLndpbmRvdywgdW5pdCkgPT4gdW5pdCA9ICJjbG9zZSIKbGV0IGNsb3NlOiB1bml0ID0+IHVuaXQgPSAoKSA9PiBnbG9iYWxUaGlzLT5jbG9zZSgpCgpsZXQgaWQ6IHVuaXQgPT4gYmlnaW50ID0gKCkgPT4gc2VsZklkCg==" download=ChobitModuleActor.res>ChobitModuleActor.res</a>
//! - <a href="data:text/plain;base64,Ly8gQ29weXJpZ2h0IChDKSAyMDI1IEhpcm9ub3JpIElzaGliYXNoaQovLwovLyBUaGlzIHdvcmsgaXMgZnJlZS4gWW91IGNhbiByZWRpc3RyaWJ1dGUgaXQgYW5kL29yIG1vZGlmeSBpdCB1bmRlciB0aGUKLy8gdGVybXMgb2YgdGhlIERvIFdoYXQgVGhlIEZ1Y2sgWW91IFdhbnQgVG8gUHVibGljIExpY2Vuc2UsIFZlcnNpb24gMiwKLy8gYXMgcHVibGlzaGVkIGJ5IFNhbSBIb2NldmFyLiBTZWUgYmVsb3cgZm9yIG1vcmUgZGV0YWlscy4KLy8KLy8gLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0tLS0KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgICAgICAgICAgICAgICAgICBWZXJzaW9uIDIsIERlY2VtYmVyIDIwMDQKLy8KLy8gQ29weXJpZ2h0IChDKSAyMDA0IFNhbSBIb2NldmFyIDxzYW1AaG9jZXZhci5uZXQ+Ci8vCi8vIEV2ZXJ5b25lIGlzIHBlcm1pdHRlZCB0byBjb3B5IGFuZCBkaXN0cmlidXRlIHZlcmJhdGltIG9yIG1vZGlmaWVkCi8vIGNvcGllcyBvZiB0aGlzIGxpY2Vuc2UgZG9jdW1lbnQsIGFuZCBjaGFuZ2luZyBpdCBpcyBhbGxvd2VkIGFzIGxvbmcKLy8gYXMgdGhlIG5hbWUgaXMgY2hhbmdlZC4KLy8KLy8gICAgICAgICAgICBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPIFBVQkxJQyBMSUNFTlNFCi8vICAgVEVSTVMgQU5EIENPTkRJVElPTlMgRk9SIENPUFlJTkcsIERJU1RSSUJVVElPTiBBTkQgTU9ESUZJQ0FUSU9OCi8vCi8vICAwLiBZb3UganVzdCBETyBXSEFUIFRIRSBGVUNLIFlPVSBXQU5UIFRPLgoKLy8gTG9hZHMgV2ViQXNzZW1ibHkgd3JpdHRlbiBieSAiY2hvYml0bGliczo6Y2hvYml0X21vZGx1ZS5ycyIuCi8vCi8vIC0gMXN0IGFyZzogUGF0aCB0byBjaG9iaXRfbW9kdWxlIHdhc20gZmlsZS4KLy8gLSBSZXR1cm46IFByb21pc2UgYWZ0ZXIgd2FzbSBmaWxlIGlzIGxvYWRlZC4KbGV0IGxvYWRXYXNtOiBzdHJpbmcgPT4gcHJvbWlzZTx1bml0PgoKLy8gU2VuZHMgbWVzc2FnZSB0byBhbm90aGVyIGFjdG9yIG9yIG1haW4gdGhyZWFkLiAgCi8vIChUaGUgbWFpbiB0aHJlYWQncyBJRCBpcyAwKSAgCi8vIChUaGUgbWVzc2FnZSBidWZmZXIgaXMgdHJhbnNmZXJlZCB0byB0aGUgYWN0b3IsIHNvIGFmdGVyIHNlbmRpbmcsIHRoZSBtZXNzYWdlIGJlY2FtZXMgZW1wdHkgb24gdGhpcyBtYWluIHRocmVhZC4pCi8vCi8vIC0gMXN0IGFyZzogQWN0b3IgSUQuCi8vIC0gMm5kIGFyZzogTWVzc2FnZS4KbGV0IHNlbmRNZXNzYWdlOiAoYmlnaW50LCBVaW50OEFycmF5LnQpID0+IHVuaXQKCi8vIENsb3NlcyB0aGlzIHRocmVhZC4KbGV0IGNsb3NlOiB1bml0ID0+IHVuaXQKCi8vIEdldHMgdGhpcyBJRC4KLy8KLy8gLSBSZXR1cm46IFRoaXMgd29ya2VyIElECmxldCBpZDogdW5pdCA9PiBiaWdpbnQK" download=ChobitModuleActor.resi>ChobitModuleActor.resi</a>
//!
//! # Interface
//!
//! - Imports from external.
//!     - Namespace is `env`
//!     - `fn init(id: u64)`
//!         - External must call this at first.
//!         - `id` : ID of this module.
//!     - `fn recv(from: usize, length: usize)`
//!         - External must call this after write data to input buffer.
//!         - `from` : Sender ID.
//!         - `length` : Message size in input buffer.
//! - Exports to external.
//!     - `fn notify_input_buffer(offset: usize, size: usize)`
//!         - This method tells information of input buffer to external.
//!         - `offset` : Address of pointer of input buffer.
//!         - `size` : Size of input buffer.
//!     - `fn notify_output_buffer(offset: usize, size: usize)`
//!         - This method tells information of output buffer to external.
//!         - `offset` : Address of pointer of output buffer.
//!         - `size` : Size of output buffer.
//!     - `fn send(to: usize, length: usize)`
//!         - External can read message from output buffer.
//!         - `to` : Receiver ID.
//!         - `length` : Message size in output buffer.
//!
//! # Example
//!
//! - `demo_wasm.wasm` written in Rust.
//! - `Demo.res` written in ReScript, depends on `ChobitModuleSystem.res`.
//! - `DemoActor.res` written in ReScript, depends on `ChobitModuleActor.res`.
//! - `demo.html` and above files are bundled by Parcel.js.
//!
//! 1. `demo.html` loads `Demo.res`.
//! 2. `Demo.res` makes ID 1 and ID 2 of `DemoActor.res`.
//! 3. Each `DemoActor.res` loads `demo_wasm.wasm`.
//! 4. Each `DemoActor.res` sends "Ready OK" message (0 length array) to `Demo.res`.
//! 5. After `Demo.res` has received these messages, `Demo.res` sends "Start!" to ID 1 `DemoActor.res`.
//! 6. ID 1 `DemoActor.res` adds " -> (MyObject: ID: 1)" to message and sends ID2 `DemoActor.res`.
//! 7. ID 2 `DemoActor.res` adds " -> (MyObject: ID: 2)" to message and sends `Demo.res`.
//! 8. `Demo.res` adds " -> Goal!" to message and outputs with `Console.log()`.
//!
//! <svg
//!    width="500"
//!    height="500"
//!    viewBox="0 0 132.29166 132.29166"
//!    version="1.1"
//!    id="svg1"
//!    xmlns="http://www.w3.org/2000/svg"
//!    xmlns:svg="http://www.w3.org/2000/svg">
//!   <defs
//!      id="defs1">
//!     <marker
//!        style="overflow:visible"
//!        id="marker9"
//!        refX="2"
//!        refY="0"
//!        orient="auto-start-reverse"
//!        markerWidth="1"
//!        markerHeight="1"
//!        viewBox="0 0 1 1"
//!        preserveAspectRatio="xMidYMid">
//!       <path
//!          transform="scale(0.5)"
//!          style="fill:context-stroke;fill-rule:evenodd;stroke:context-stroke;stroke-width:1pt"
//!          d="M 5.77,0 -2.88,5 V -5 Z"
//!          id="path9" />
//!     </marker>
//!     <marker
//!        style="overflow:visible"
//!        id="marker8"
//!        refX="2"
//!        refY="0"
//!        orient="auto-start-reverse"
//!        markerWidth="1"
//!        markerHeight="1"
//!        viewBox="0 0 1 1"
//!        preserveAspectRatio="xMidYMid">
//!       <path
//!          transform="scale(0.5)"
//!          style="fill:context-stroke;fill-rule:evenodd;stroke:context-stroke;stroke-width:1pt"
//!          d="M 5.77,0 -2.88,5 V -5 Z"
//!          id="path8" />
//!     </marker>
//!     <marker
//!        style="overflow:visible"
//!        id="Triangle"
//!        refX="2"
//!        refY="0"
//!        orient="auto-start-reverse"
//!        markerWidth="1"
//!        markerHeight="1"
//!        viewBox="0 0 1 1"
//!        preserveAspectRatio="xMidYMid">
//!       <path
//!          transform="scale(0.5)"
//!          style="fill:context-stroke;fill-rule:evenodd;stroke:context-stroke;stroke-width:1pt"
//!          d="M 5.77,0 -2.88,5 V -5 Z"
//!          id="path135" />
//!     </marker>
//!   </defs>
//!   <g
//!      id="layer2"
//!      transform="scale(0.49999996)"
//!      style="stroke-width:2">
//!     <rect
//!        style="fill:#ffffff;stroke:none;stroke-width:0.6;stroke-linejoin:round;stroke-dashoffset:5.66928"
//!        id="rect1"
//!        width="264.58334"
//!        height="264.58334"
//!        x="0"
//!        y="0" />
//!   </g>
//!   <g
//!      id="g2"
//!      style="fill:#808080;stroke:none;stroke-width:2.82553;stroke-dasharray:none"
//!      transform="matrix(0.93640356,0,0,0.93640356,113.14418,130.8411)">
//!     <g
//!        id="g14"
//!        transform="matrix(0.4266667,0,0,0.4266667,-58.914709,-69.750031)"
//!        style="stroke-width:6.62232">
//!       <circle
//!          style="fill:#ffffff;stroke:#000000;stroke-width:6.6223;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          id="path1"
//!          cx="91.086067"
//!          cy="72.187248"
//!          r="35.319084"
//!          transform="translate(-70.638169,-141.27634)" />
//!       <circle
//!          style="fill:#ffffff;stroke:#000000;stroke-width:6.6223;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          id="circle1"
//!          cx="91.086067"
//!          cy="72.187248"
//!          r="35.319084"
//!          transform="translate(-141.27634)" />
//!       <circle
//!          style="fill:#ffffff;stroke:#000000;stroke-width:6.6223;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          id="circle2"
//!          cx="91.086067"
//!          cy="72.187248"
//!          r="35.319084" />
//!       <path
//!          style="fill:#808080;stroke:#000000;stroke-width:6.62232;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928;marker-end:url(#Triangle)"
//!          d="M 4.6529205,-37.499144 -34.311007,40.639085"
//!          id="path5" />
//!       <path
//!          style="fill:#808080;stroke:#000000;stroke-width:6.62232;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928;marker-end:url(#marker8)"
//!          d="M -14.871189,72.187248 H 55.766983"
//!          id="path6" />
//!       <path
//!          style="fill:#808080;stroke:#000000;stroke-width:6.62232;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928;marker-end:url(#marker9)"
//!          d="M 75.290896,40.596897 36.243064,-37.498738"
//!          id="path7" />
//!     </g>
//!   </g>
//!   <g
//!      id="layer3">
//!     <text
//!        xml:space="preserve"
//!        style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;line-height:normal;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#000000;stroke:none;stroke-width:2.64583;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!        x="54.255699"
//!        y="13.16755"
//!        id="text9"><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="54.255699"
//!          y="13.16755"
//!          id="tspan10">Demo.res</tspan><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="54.255699"
//!          y="19.417538"
//!          id="tspan11">ID: 0</tspan></text>
//!     <text
//!        xml:space="preserve"
//!        style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;line-height:normal;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#000000;stroke:none;stroke-width:2.64583;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!        x="13.867557"
//!        y="116.41422"
//!        id="text13"><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="13.867557"
//!          y="116.41422"
//!          id="tspan12">DemoActor.res</tspan><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="13.867557"
//!          y="122.66421"
//!          id="tspan14">demo_wasm.wasm</tspan><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="13.867557"
//!          y="128.9142"
//!          id="tspan13">ID: 1</tspan></text>
//!     <text
//!        xml:space="preserve"
//!        style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;line-height:normal;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#000000;stroke:none;stroke-width:2.64583;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!        x="71.193886"
//!        y="116.41422"
//!        id="text17"><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="71.193886"
//!          y="116.41422"
//!          id="tspan15">DemoActor.res</tspan><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="71.193886"
//!          y="122.66421"
//!          id="tspan16">demo_wasm.wasm</tspan><tspan
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:4.99999px;font-family:'Noto Sans CJK JP';-inkscape-font-specification:'Noto Sans CJK JP, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#000000;stroke:none;stroke-width:2.64583"
//!          x="71.193886"
//!          y="128.9142"
//!          id="tspan17">ID: 2</tspan></text>
//!   </g>
//!   <g
//!      id="layer4">
//!     <g
//!        id="g22"
//!        transform="matrix(0.49655998,0,0,0.4946504,4.6987131,-9.6473714)"
//!        style="stroke-width:2.01774">
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="82.37159"
//!          y="128.377"
//!          id="text22"><tspan
//!            id="tspan22"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-dasharray:none"
//!            x="82.37159"
//!            y="128.377">&quot;Start!&quot;</tspan></text>
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:none;stroke-width:5.33859;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="82.37159"
//!          y="128.377"
//!          id="text18"><tspan
//!            id="tspan18"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke-width:5.33859"
//!            x="82.37159"
//!            y="128.377">&quot;Start!&quot;</tspan></text>
//!     </g>
//!     <g
//!        id="g23"
//!        transform="matrix(0.49655998,0,0,0.4946504,-11.836314,-1.4116514)"
//!        style="stroke-width:2.01774">
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="92.887115"
//!          y="206.36714"
//!          id="text23"><tspan
//!            id="tspan23"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-dasharray:none"
//!            x="92.887115"
//!            y="206.36714">&quot;Start! -&gt; (MyObject: id: 1)&quot;</tspan></text>
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:none;stroke-width:5.33859;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="92.887115"
//!          y="206.36714"
//!          id="text19"><tspan
//!            id="tspan19"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:5px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke-width:5.33859"
//!            x="92.887115"
//!            y="206.36714">&quot;Start! -&gt; (MyObject: id: 1)&quot;</tspan></text>
//!     </g>
//!     <g
//!        id="g24"
//!        transform="matrix(0.49655998,0,0,0.4946504,-0.0877419,4.8735036)"
//!        style="stroke-width:2.01774">
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="110.85114"
//!          y="146.77916"
//!          id="text24"><tspan
//!            id="tspan24"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-dasharray:none"
//!            x="110.85114"
//!            y="146.77916">&quot;Start! -&gt; (MyObject: id: 1) -&gt; (MyObject: id: 2)&quot;</tspan></text>
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:none;stroke-width:5.33859;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="110.85114"
//!          y="146.77916"
//!          id="text20"><tspan
//!            id="tspan20"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke-width:5.33859"
//!            x="110.85114"
//!            y="146.77916">&quot;Start! -&gt; (MyObject: id: 1) -&gt; (MyObject: id: 2)&quot;</tspan></text>
//!     </g>
//!     <g
//!        id="g25"
//!        transform="matrix(0.49655998,0,0,0.4946504,1.0000891,-2.7120284)"
//!        style="stroke-width:2.01774">
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="82.809746"
//!          y="90.696381"
//!          id="text25"><tspan
//!            id="tspan25"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke:#ffffff;stroke-width:2.66931;stroke-dasharray:none"
//!            x="82.809746"
//!            y="90.696381">&quot;Start! -&gt; (MyObject: id: 1) -&gt; (MyObject: id: 2) -&gt; Goal!&quot;</tspan></text>
//!       <text
//!          xml:space="preserve"
//!          style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;line-height:normal;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;text-align:start;text-decoration-color:#000000;letter-spacing:0px;writing-mode:lr-tb;direction:ltr;text-anchor:start;fill:#ff0000;stroke:none;stroke-width:5.33859;stroke-linejoin:round;stroke-dasharray:none;stroke-dashoffset:5.66928"
//!          x="82.809746"
//!          y="90.696381"
//!          id="text21"><tspan
//!            id="tspan21"
//!            style="font-style:normal;font-variant:normal;font-weight:bold;font-stretch:normal;font-size:6px;font-family:'Ricty Diminished Discord';-inkscape-font-specification:'Ricty Diminished Discord, Bold';font-variant-ligatures:normal;font-variant-caps:normal;font-variant-numeric:normal;font-variant-east-asian:normal;fill:#ff0000;stroke-width:5.33859"
//!            x="82.809746"
//!            y="90.696381">&quot;Start! -&gt; (MyObject: id: 1) -&gt; (MyObject: id: 2) -&gt; Goal!&quot;</tspan></text>
//!     </g>
//!   </g>
//! </svg>
//!
//! ### file: `demo_wasm.wasm`
//! 
//! ```ignore
//! extern crate chobitlibs;
//! 
//! use chobitlibs::chobit_module::*;
//! 
//! struct MyObject {
//!   pub value: String
//! }
//! 
//! chobit_module! {
//!   input_buffer_size = 16 * 1024;
//!   output_buffer_size = 16 * 1024;
//! 
//!   on_created(my_id: u64) -> MyObject {
//!     MyObject {
//!       value: format!(" -> (MyObject: id: {})", my_id)
//!     }
//!   }
//! 
//!   on_received(module: &mut ChobitModule<MyObject>) {
//!     let mut data = module.recv_data().1.to_vec();
//! 
//!     data.extend_from_slice(module.user_object().value.as_bytes());
//! 
//!     let next_id = if module.id() >= 2 {
//!       0
//!     } else {
//!       module.id() + 1
//!     };
//! 
//!     module.send(next_id, data.as_slice())
//!   }
//! }
//! ```
//! 
//! ### file: `Demo.res`
//! 
//! This file is compiled to `Demo.res.mjs` by ReScript and then compiled to `Demo.res.js` by Parcel.js.
//! 
//! ```text
//! open ChobitModuleSystem
//! 
//! type textEncoder
//! @new external createTextEncoder: unit => textEncoder = "TextEncoder"
//! @send external encode: (textEncoder, string) => Uint8Array.t = "encode"
//! let encoder = createTextEncoder()
//! 
//! type textDecoder
//! @new external createTextDecoder: string => textDecoder = "TextDecoder"
//! @send external decode: (textDecoder, Uint8Array.t) => string = "decode"
//! let decoder = createTextDecoder("utf-8")
//! 
//! let actor1Id = 1n
//! let actor2Id = 2n
//! 
//! let actor1Ok: ref<bool> = ref(false)
//! let actor2Ok: ref<bool> = ref(false)
//! 
//! let onReceived: (bigint, Uint8Array.t) => unit = (senderId, data) => {
//!   if data->TypedArray.length === 0 {
//!     if senderId === actor1Id {
//!       actor1Ok := true
//!     } else if senderId === actor2Id {
//!       actor2Ok := true
//!     }
//! 
//!     if actor1Ok.contents && actor2Ok.contents {
//!       sendMessage(1n, encoder->encode("Start!"))
//!     }
//!   } else {
//!     Console.log(decoder->decode(data) ++ " -> Goal!")
//!   }
//! }
//! 
//! addActor("DemoActor.res.js", actor1Id, onReceived)
//! addActor("DemoActor.res.js", actor2Id, onReceived)
//! ```
//! 
//! ### file: `DemoActor.res`
//! 
//! This file is compiled to `DemoActor.res.mjs` by ReScript and then compiled to `DemoActor.res.js` by Parcel.js.
//! 
//! ```text
//! open ChobitModuleActor
//! 
//! loadWasm("demo_wasm.wasm")->Promise.thenResolve((_) => {
//!   Console.log("Actor ID " ++ id()->BigInt.toString ++ " is OK!")
//! 
//!   sendMessage(0n, Uint8Array.fromLength(0))
//! })->ignore
//! ```
//! 
//! ### file: `demo.html`
//! 
//! ```text
//! <!DOCTYPE html>
//! <html lang="ja">
//! <head>
//!   <meta charset="UTF-8">
//!   <script defer type="module" src="Demo.res.mjs"></script>
//!   <title>Demo</title>
//! </head>
//! <body>
//!   
//! </body>
//! </html>
//! ```
//! 
//! 
//! 
//! 
//! 
//!

use alloc::{boxed::Box, vec};

#[link(wasm_import_module = "env")]
extern "C" {
    fn notify_input_buffer(offset: usize, size: usize);
    fn notify_output_buffer(offset: usize, size: usize);

    fn send(to: u64, length: usize);
}

/// An object that has all information and data of WASM.
pub struct ChobitModule<T> {
    id: u64,

    input_buffer: Box<[u8]>,
    output_buffer: Box<[u8]>,

    recv_from: u64,
    recv_length: usize,

    user_object: T
}

impl<T> ChobitModule<T> {
    /// Gets module ID.
    ///
    /// ID is given from runtime when the module is initialized.
    ///
    /// - _Return_ : Module ID.
    #[inline]
    pub fn id(&self) -> u64 {self.id}

    /// Gets input buffer size.
    ///
    /// Input buffer is a buffer that is put input data from runtime.
    ///
    /// - _Return_ : A size of input buffer.
    #[inline]
    pub fn input_buffer_size(&self) -> usize {(*self.input_buffer).len()}

    /// Gets output buffer size.
    ///
    /// Output buffer is a buffer that the module puts output data.
    ///
    /// - _Return_ : A size of output buffer.
    #[inline]
    pub fn output_buffer_size(&self) -> usize {(*self.output_buffer).len()}

    /// Gets recieved data from other module.
    ///
    /// - _Return_ : (other_module_id, data)
    #[inline]
    pub fn recv_data(&self) -> (u64, &[u8]) {
        (self.recv_from, &(*self.input_buffer)[..self.recv_length])
    }

    fn copy_to_output_buffer(&mut self, data: &[u8]) -> usize {
        let data_len = data.len().min(self.output_buffer.len());

        (*self.output_buffer)[..data_len].copy_from_slice(data);

        data_len
    }

    /// Sends data to other module.
    ///
    /// - `to` : Other module ID.
    /// - `data` : Data that you want to send.
    #[inline]
    pub fn send(&mut self, to: u64, data: &[u8]) {
        unsafe {
            send(to, self.copy_to_output_buffer(data));
        }
    }

    /// Resizes input buffer.
    ///
    /// - `size` : New size of input buffer.
    pub fn resize_input_buffer(&mut self, size: usize) {
        let buffer = vec![0u8; size].into_boxed_slice();
        let offset = (*buffer).as_ptr() as usize;
        let size = (*buffer).len();

        unsafe {
            notify_input_buffer(offset, size);
        }

        self.input_buffer = buffer;
    }

    /// Resizes output buffer.
    ///
    /// - `size` : New size of output buffer.
    pub fn resize_output_buffer(&mut self, size: usize) {
        let buffer = vec![0u8; size].into_boxed_slice();
        let offset = (*buffer).as_ptr() as usize;
        let size = (*buffer).len();

        unsafe {
            notify_output_buffer(offset, size);
        }

        self.output_buffer = buffer;
    }

    /// Gets immutable user object.
    ///
    /// - _Return_ : Immutable user object.
    #[inline]
    pub fn user_object(&self) -> &T {&self.user_object}

    /// Gets mutable user object.
    ///
    /// - _Return_ : Mutable user object.
    #[inline]
    pub fn user_object_mut(&mut self) -> &mut T {&mut self.user_object}

    #[doc(hidden)]
    pub fn __new(
        id: u64,
        input_buffer_size: usize,
        output_buffer_size: usize,
        user_object: T
    ) -> Self {
        let ret = Self {
            id: id,

            input_buffer:
                vec![0u8; input_buffer_size].into_boxed_slice(),
            output_buffer:
                vec![0u8; output_buffer_size].into_boxed_slice(),

            recv_from: 0,
            recv_length: 0,

            user_object: user_object
        };

        unsafe {
            notify_input_buffer(
                (*ret.input_buffer).as_ptr() as usize,
                (*ret.input_buffer).len(),
            );

            notify_output_buffer(
                (*ret.output_buffer).as_ptr() as usize,
                (*ret.output_buffer).len(),
            );

            ret
        }
    }

    #[doc(hidden)]
    pub fn __set_recv_info(&mut self, from: u64, length: usize) {
        self.recv_from = from;
        self.recv_length = length;
    }
}

/// Defines WASM module. Defined in _chobit_module.rs_ .
///
/// ```ignore
/// use chobitlibs::chobit_module::{ChobitModule, chobit_module};
///
/// struct MyObject {
///     pub value: i32
/// }
///
/// chobit_module! {
///     input_buffer_size = 16 * 1024;  // Initial input buffer size.
///     output_buffer_size = 16 * 1024;  // Initial output buffer size.
///
///     // This is called when this module has created.
///     on_created() -> MyObject {
///         MyObject {
///             value: 100
///         }
///     }
///
///     // This is called when received data from other module.
///     on_received(module: &mut ChobitModule<MyObject>) {
///         module.send(
///             123,
///             format!("Hello {}", module.user_object().value).as_bytes()
///         );
///     }
/// }
/// ```
#[macro_export]
macro_rules! chobit_module {
    (
        input_buffer_size = $input_buffer_size:expr;
        output_buffer_size = $output_buffer_size:expr;

        on_created($($on_created_args:tt)*) -> $user_object_type:ty {
            $($code_1:tt)*
        }

        on_received($($on_received_args:tt)*) {
            $($code_2:tt)*
        }
    ) => {
static mut __MODULE: Option<ChobitModule<$user_object_type>> = None;

fn __on_created($($on_created_args)*) -> $user_object_type {
    $($code_1)*
}

fn __on_received($($on_received_args)*) {
    $($code_2)*
}

#[allow(dead_code)]
#[no_mangle]
extern "C" fn init(id: u64) {
    unsafe {
        __MODULE = Some(ChobitModule::<$user_object_type>::__new(
            id,
            $input_buffer_size,
            $output_buffer_size,
            __on_created(id)
        ));
    }
}

#[allow(dead_code)]
#[no_mangle]
extern "C" fn recv(from: u64, length: usize) {
    match unsafe {__MODULE.as_mut()} {
        Some(module) => {
            module.__set_recv_info(from, length);

            __on_received(module);
        },

        None => {}
    }
}
    };
}
pub use chobit_module;
