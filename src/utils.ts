import moment from 'moment';

export function bytesHumanReadable(bytes: number): string {
  if (bytes <= 1024) {
    return bytes + ' B';
  } else if (bytes <= 1024 * 1024) {
    return (bytes / 1024).toFixed(2) + ' KB';
  } else if (bytes <= 1024 * 1024 * 1024) {
    return (bytes / 1024 / 1024).toFixed(2) + ' MB';
  } else if (bytes <= 1024 * 1024 * 1024 * 1024) {
    return (bytes / 1024 / 1024 / 1024).toFixed(2) + ' GB';
  } else {
    return (bytes / 1024 / 1024 / 1024 / 1024).toFixed(2) + ' TB';
  }
  return '0 B';
}

export function timeHumanReadable(seconds: number): string {
  let m = moment.duration(seconds, 'seconds');
  return m.humanize();
}
