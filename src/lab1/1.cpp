#include <algorithm>
#include <fstream>
#include <iostream>
#include <list>
#include <sstream>
#include <stdexcept>
#include <string>
#include <vector>

// Section set class ==========================================================
class set final {
public:
  set &add(char x);
  set &remove(char x);
  set &clear();

  std::vector<set> power() const;

  set &operator+=(set const &other) &;
  set operator+(set const &other) const;

  set &operator&=(set const &other) &;
  set operator&(set const &other) const;

  set &operator-=(set const &other) &;
  set operator-(set const &other) const;

  bool operator<=(set const &other) const;
  bool operator==(set const &other) const;

  std::string to_string() const;

private:
  std::list<char> m_elements;
};

set &set::add(char x) {
  auto it = m_elements.begin();
  for (; it != m_elements.end(); ++it) {
    if (*it == x) {
      return *this;
    }
    if (*it > x) {
      break;
    }
  }
  m_elements.insert(it, x);

  return *this;
}

set &set::remove(char x) {
  m_elements.remove(x);
  return *this;
}

set &set::clear() {
  m_elements.clear();
  return *this;
}

std::vector<set> set::power() const {
  if (m_elements.size() > sizeof(unsigned long long) << 3) {
    throw std::runtime_error("Elements count in set is too big");
  }
  std::vector<set> result;

  int n = m_elements.size();
  unsigned long long N = 1ULL << n;

  for (unsigned long long mask = 0; mask < N; ++mask) {
    set subset;
    auto it = m_elements.begin();
    for (int i = 0; i < n; ++i, ++it) {
      if (mask & (1ULL << i)) {
        subset.m_elements.push_back(*it);
      }
    }
    result.push_back(subset);
  }

  return result;
}

set &set::operator+=(set const &other) & { return *this = *this + other; }

set set::operator+(set const &other) const {
  set merged;
  auto it1 = m_elements.begin();
  auto it2 = other.m_elements.begin();
  while (it1 != m_elements.end() && it2 != other.m_elements.end()) {
    if (*it1 < *it2) {
      merged.m_elements.push_back(*it1);
      ++it1;
    } else if (*it2 < *it1) {
      merged.m_elements.push_back(*it2);
      ++it2;
    } else {
      merged.m_elements.push_back(*it1);
      ++it1;
      ++it2;
    }
  }

  while (it1 != m_elements.end()) {
    merged.m_elements.push_back(*it1);
    ++it1;
  }

  while (it2 != other.m_elements.end()) {
    merged.m_elements.push_back(*it2);
    ++it2;
  }

  return merged;
}

set &set::operator&=(set const &other) & { return *this = *this & other; }

set set::operator&(set const &other) const {
  set merged;
  auto it1 = m_elements.begin();
  auto it2 = other.m_elements.begin();
  while (it1 != m_elements.end() && it2 != other.m_elements.end()) {
    if (*it1 < *it2) {
      ++it1;
    } else if (*it2 < *it1) {
      ++it2;
    } else {
      merged.m_elements.push_back(*it1);
      ++it1;
      ++it2;
    }
  }

  return merged;
}

set &set::operator-=(set const &other) & { return *this = *this - other; }

set set::operator-(set const &other) const {
  set merged;
  auto it1 = m_elements.begin();
  auto it2 = other.m_elements.begin();
  while (it1 != m_elements.end() && it2 != other.m_elements.end()) {
    if (*it1 < *it2) {
      merged.m_elements.push_back(*it1);
      ++it1;
    } else if (*it2 < *it1) {
      ++it2;
    } else {
      ++it1;
      ++it2;
    }
  }

  while (it1 != m_elements.end()) {
    merged.m_elements.push_back(*it1);
    ++it1;
  }

  return merged;
}

bool set::operator<=(set const &other) const {
  auto it1 = m_elements.begin();
  auto it2 = other.m_elements.begin();

  while (it1 != m_elements.end() && it2 != other.m_elements.end()) {
    if (*it1 == *it2) {
      ++it1;
      ++it2;
    } else if (*it1 > *it2) {
      ++it2;
    } else {
      return false;
    }
  }

  return it1 == m_elements.end();
}

bool set::operator==(set const &other) const {
  return m_elements == other.m_elements;
}

std::string set::to_string() const {
  std::ostringstream out;
  out << "{";
  bool first = true;
  for (char c : m_elements) {
    if (!first)
      out << ",";
    out << c;
    first = false;
  }
  out << "}";
  return out.str();
}

// Section main ===============================================================

set *find_set(std::vector<std::pair<char, set>> &sets, char name) {
  for (auto &p : sets) {
    if (p.first == name)
      return &p.second;
  }
  return nullptr;
}

void handle_command(std::vector<std::pair<char, set>> &sets,
                    const std::string &line) {
  std::istringstream iss(line);
  std::string cmd;
  if (!(iss >> cmd))
    throw std::runtime_error("Empty command");

  if (cmd == "new") {
    char name;
    if (!(iss >> name))
      throw std::runtime_error("'new' requires set name");
    if (!find_set(sets, name)) {
      sets.emplace_back(name, set());
      std::cout << "New set " << name << "\n";
    } else {
      throw std::runtime_error(std::string("Set ") + name + " already exists");
    }
  } else if (cmd == "del") {
    char name;
    if (!(iss >> name))
      throw std::runtime_error("'del' requires set name");
    auto it = std::remove_if(sets.begin(), sets.end(),
                             [name](auto &p) { return p.first == name; });
    if (it != sets.end() && it != sets.begin()) {
      sets.erase(it, sets.end());
      std::cout << "Deleted set " << name << "\n";
    } else {
      throw std::runtime_error(std::string("Set ") + name + " not found");
    }
  } else if (cmd == "add") {
    char name, x;
    if (!(iss >> name >> x))
      throw std::runtime_error("'add' requires set name and element");
    if (auto s = find_set(sets, name)) {
      s->add(x);
      std::cout << "Added '" << x << "' to " << name << "\n";
    } else {
      throw std::runtime_error(std::string("Set ") + name + " not found");
    }
  } else if (cmd == "rem") {
    char name, x;
    if (!(iss >> name >> x))
      throw std::runtime_error("'rem' requires set name and element");
    if (auto s = find_set(sets, name)) {
      s->remove(x);
      std::cout << "Removed '" << x << "' from " << name << "\n";
    } else {
      throw std::runtime_error(std::string("Set ") + name + " not found");
    }
  } else if (cmd == "pow") {
    char name;
    if (!(iss >> name))
      throw std::runtime_error("'pow' requires set name");
    if (auto s = find_set(sets, name)) {
      auto P = s->power();
      std::cout << "Power set of " << name << ": {\n";
      for (auto &sub : P) {
        std::cout << "  " << sub.to_string() << ",\n";
      }
      std::cout << "}\n";
    } else {
      throw std::runtime_error(std::string("Set ") + name + " not found");
    }
  } else if (cmd == "see") {
    char name;
    if (iss >> name) {
      if (auto s = find_set(sets, name))
        std::cout << name << ": " << s->to_string() << "\n";
      else
        throw std::runtime_error(std::string("Set ") + name + " not found");
    } else {
      for (auto &p : sets)
        std::cout << p.first << ": " << p.second.to_string() << "\n";
    }
  } else {
    // binary operations
    char name1, name2;
    std::string op;
    std::istringstream iss2(line);
    if (!(iss2 >> name1 >> op >> name2))
      throw std::runtime_error("Invalid binary operation format");

    auto s1 = find_set(sets, name1);
    auto s2 = find_set(sets, name2);
    if (!s1 || !s2) {
      if (!s1)
        throw std::runtime_error(std::string("Set ") + name1 + " not found");
      if (!s2)
        throw std::runtime_error(std::string("Set ") + name2 + " not found");
    }

    if (op == "+") {
      auto result = *s1 + *s2;
      std::cout << name1 << " + " << name2 << " = " << result.to_string()
                << "\n";
    } else if (op == "&") {
      auto result = *s1 & *s2;
      std::cout << name1 << " & " << name2 << " = " << result.to_string()
                << "\n";
    } else if (op == "-") {
      auto result = *s1 - *s2;
      std::cout << name1 << " - " << name2 << " = " << result.to_string()
                << "\n";
    } else if (op == "<") {
      std::cout << name1 << " < " << name2 << " ? "
                << ((*s1 <= *s2) ? "true" : "false") << "\n";
    } else if (op == "=") {
      std::cout << name1 << " = " << name2 << " ? "
                << ((*s1 == *s2) ? "true" : "false") << "\n";
    } else {
      throw std::runtime_error(std::string("Unknown operator '") + op + "'");
    }
  }
}

int main(int argc, char *argv[]) {
  if (argc != 2) {
    std::cerr << "Incorrect usage!\n";
    return 1;
  }

  std::vector<std::pair<char, set>> sets;
  std::ifstream file(argv[1]);
  if (!file.is_open()) {
    std::cerr << "Cannot open file\n";
    return 1;
  }

  std::string line;
  while (std::getline(file, line)) {
    if (!line.empty()) {
      try {
        handle_command(sets, line);
      } catch (const std::runtime_error &e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
      }
    }
  }

  return 0;
}
