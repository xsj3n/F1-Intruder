import MillionLint from '@million/lint';
/** @type {import('next').NextConfig} */
const nextConfig = {
  output: 'export'
};
export default MillionLint.next({
  rsc: true
})(nextConfig);