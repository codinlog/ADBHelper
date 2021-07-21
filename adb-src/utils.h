#pragma once
#include <iostream>
#include <string>
#include <vector>
template <typename... Args>
std::string StringPrintf(const char* format, Args... args) {
	int length = std::snprintf(nullptr, 0, format, args...);
	//   assert(length >= 0);

	char* buf = new char[length + 1];
	std::snprintf(buf, length + 1, format, args...);

	std::string str(buf);
	delete[] buf;
	return str;
}

inline std::vector<std::string> Split(const std::string& str, const std::string& sp) {
	std::vector<std::string> v{};
	std::string::size_type pos1{ 0 }, pos2{ str.find(sp) };
	while (std::string::npos != pos2)
	{
		auto diff = pos2 - pos1;
		if (diff > 1) {
			v.push_back(std::move(str.substr(pos1, diff)));
		}
		pos1 = pos2 + sp.size();
		pos2 = str.find(sp, pos1);
	}
	if (pos1 != str.length()) {
		v.push_back(str.substr(pos1));
	}
	return v;
}

inline std::string Join(const std::vector<std::string>& v, const std::string& sp) {
	std::string s{};
	for (const auto& t : v) {
		s = s.append(t).append(sp);
	}
	return s;
}

inline std::string Join(const std::vector<std::string>& v, const char sp) {
	std::string s{};
	std::string s_sp{ sp };
	for (const auto& t : v) {
		s = s.append(t).append(s_sp);
	}
	return s;
}

inline std::string SystemErrorCodeToString(DWORD d) {
	return std::to_string(long long(d));
}
