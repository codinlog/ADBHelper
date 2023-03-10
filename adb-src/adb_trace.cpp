/*
 * Copyright (C) 2015 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <vector>
#include <unordered_map>
#include <iostream>
#include <string>
#include "sysdeps.h"
#include "adb_trace.h"
#include "adb.h"

#if !ADB_HOST
const char* adb_device_banner = "device";
static android::base::LogdLogger gLogdLogger;
#else
const char* adb_device_banner = "host";
#endif

int adb_trace_mask;

std::string get_trace_setting_from_env() {
	const char* setting = getenv("ADB_TRACE");
	if (setting == nullptr) {
		setting = "";
	}

	return std::string(setting);
}

std::string get_trace_setting() {
#if ADB_HOST
	return get_trace_setting_from_env();
#else
#endif
}

// Split the space separated list of tags from the trace setting and build the
// trace mask from it. note that '1' and 'all' are special cases to enable all
// tracing.
//
// adb's trace setting comes from the ADB_TRACE environment variable, whereas
// adbd's comes from the system property persist.adb.trace_mask.
static void setup_trace_mask() {
	const std::string trace_setting = get_trace_setting();
	if (trace_setting.empty()) {
		return;
	}

	std::unordered_map<std::string, int> trace_flags = {
		{"1", -1},
		{"all", -1},
		{"adb", ADB},
		{"sockets", SOCKETS},
		{"packets", PACKETS},
		{"rwx", RWX},
		{"usb", USB},
		{"sync", SYNC},
		{"sysdeps", SYSDEPS},
		{"transport", TRANSPORT},
		{"jdwp", JDWP},
		{"services", SERVICES},
		{"auth", AUTH},
		{"fdevent", FDEVENT},
		{"shell", SHELL} };

	std::vector<std::string> elements = android::base::Split(trace_setting, " ");
	for (const auto& elem : elements) {
		const auto& flag = trace_flags.find(elem);
		if (flag == trace_flags.end()) {
			std::cout << "Unknown trace flag: " << elem;
			continue;
		}

		if (flag->second == -1) {
			// -1 is used for the special values "1" and "all" that enable all
			// tracing.
			adb_trace_mask = ~0;
			return;
		}
		else {
			adb_trace_mask |= 1 << flag->second;
		}
	}
}

void adb_trace_init(char** argv) {
#if !ADB_HOST
	// Don't open log file if no tracing, since this will block
	// the crypto unmount of /data
	if (!get_trace_setting().empty()) {
		if (unix_isatty(STDOUT_FILENO) == 0) {
			start_device_log();
		}
	}
#endif

#if ADB_HOST && !defined(_WIN32)
	// adb historically ignored $ANDROID_LOG_TAGS but passed it through to logcat.
	// If set, move it out of the way so that libbase logging doesn't try to parse it.
	std::string log_tags;
	char* ANDROID_LOG_TAGS = getenv("ANDROID_LOG_TAGS");
	if (ANDROID_LOG_TAGS) {
		log_tags = ANDROID_LOG_TAGS;
		unsetenv("ANDROID_LOG_TAGS");
	}
#endif

	android::base::InitLogging(argv, &AdbLogger);

#if ADB_HOST && !defined(_WIN32)
	// Put $ANDROID_LOG_TAGS back so we can pass it to logcat.
	if (!log_tags.empty()) setenv("ANDROID_LOG_TAGS", log_tags.c_str(), 1);
#endif

	setup_trace_mask();

	VLOG(ADB) << adb_version();
}

void adb_trace_enable(AdbTrace trace_tag) {
	adb_trace_mask |= (1 << trace_tag);
}