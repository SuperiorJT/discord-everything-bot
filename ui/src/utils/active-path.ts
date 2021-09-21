export const isActivePath = (path: string, target: string): boolean => {
  const regex = new RegExp(`^${escapeRegExp(target)}(\\?.*)*$`);
  return !!regex.exec(path);
};

export const isPartialPath = (path: string, target: string): boolean => {
  const regex = new RegExp(`^${escapeRegExp(target)}(\\/.*)*(\\?.*)*$`);
  return !!regex.exec(path);
};

function escapeRegExp(string) {
  return string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'); // $& means the whole matched string
}
