const AllowedKeys = [
  'selectedFilePath',
  'lastEvent',
  'currentView',
  'currentDiskLetter',
  'currentPath',
  'scopedFolder'
];

function SetKeyLocalStorage(key, value) {
  if (AllowedKeys.includes(key)) {
    if (value !== undefined && value !== null) {
      localStorage.setItem(key, value);
    } else {
      // value is undefined or null
      console.warn(`Value for key "${key}" is undefined or null.`);
    }
  } else {
    // key is not allowed in AllowedKeys
    console.warn(`Key "${key}" is not allowed.`);
  }
}

function SetObjectLocalStorage(key, value, tradeHash) {
  if (AllowedKeys.includes(key)) {
    if (value !== undefined && value !== null) {
      const jsonObject = {
        value: value,
        tradeHash: tradeHash,
      };
      localStorage.setItem(key, JSON.stringify(jsonObject));
    }
  }
}

function GetObjectLocalStorage(key) {
  if (AllowedKeys.includes(key)) {
    const jsonObject = JSON.parse(localStorage.getItem(key));
    return jsonObject;
  } else {
    console.warn(`Key "${key}" is not allowed.`);
    return null;
  }
}

function RemoveObjectLocalStorage(key) {
  if (AllowedKeys.includes(key)) {
    localStorage.removeItem(key);
  } else {
    console.warn(`Key "${key}" is not allowed.`);
  }
}

function GetKeyLocalStorage(key) {
  if (typeof localStorage !== 'undefined') {
    if (AllowedKeys.includes(key)) {
      return localStorage.getItem(key);
    } else {
      console.warn(`Key "${key}" is not allowed.`);
      return null;
    }
  } else {
    console.warn('localStorage is not defined.');
    return null;
  }
}

function RemoveKeyLocalStorage(key) {
  if (AllowedKeys.includes(key)) {
    localStorage.removeItem(key);
  } else {
    console.warn(`Key "${key}" is not allowed.`);
  }
}

function RemoveAllCachedKeys() {
  AllowedKeys.filter((key) => !key.startsWith('cached')).forEach((key) => {
    localStorage.removeItem(key);
  });
}

function clearAll() {
  if (typeof localStorage !== 'undefined') {
    localStorage.clear();
  }
}

export {
  SetKeyLocalStorage,
  GetKeyLocalStorage,
  RemoveKeyLocalStorage,
  RemoveAllCachedKeys,
  SetObjectLocalStorage,
  GetObjectLocalStorage,
  RemoveObjectLocalStorage,
  clearAll,
};
