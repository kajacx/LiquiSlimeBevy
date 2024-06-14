export const settings = {
  none: () => NoneSettings,
};

interface Settings {
  describeSettings: () => void;
}

class NoneSettings implements Settings {
  describeSettings = () => void {};
}
