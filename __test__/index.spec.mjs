import test from 'ava';
import { SrpInteger } from '../index.js';

test('SrpInteger.fromHex and toHex', (t) => {
  const hex = 'abcdef123456';
  const integer = SrpInteger.fromHex(hex);
  t.is(integer.toHex(), hex);
});

test('SrpInteger.add', (t) => {
  const a = SrpInteger.fromHex('1');
  const b = SrpInteger.fromHex('2');
  const result = a.add(b);
  t.is(result.toHex(), '3');
});

test('SrpInteger.subtract', (t) => {
  const a = SrpInteger.fromHex('5');
  const b = SrpInteger.fromHex('3');
  const result = a.subtract(b);
  t.is(result.toHex(), '2');
});

test('SrpInteger.multiply', (t) => {
  const a = SrpInteger.fromHex('2');
  const b = SrpInteger.fromHex('4');
  const result = a.multiply(b);
  t.is(result.toHex(), '8');
});

test('SrpInteger.modPow', (t) => {
  const base = SrpInteger.fromHex('2');
  const exponent = SrpInteger.fromHex('3');
  const modulus = SrpInteger.fromHex('5');
  const result = base.modPow(exponent, modulus);
  t.is(result.toHex(), '3'); // 2^3 % 5 = 8 % 5 = 3
});

test('SrpInteger.modulo', (t) => {
  const a = SrpInteger.fromHex('a');
  const mod = SrpInteger.fromHex('3');
  const result = a.modulo(mod);
  t.is(result.toHex(), '1'); // a (10) % 3 = 1
});

test('SrpInteger.equals', (t) => {
  const a = SrpInteger.fromHex('123');
  const b = SrpInteger.fromHex('123');
  const c = SrpInteger.fromHex('321');
  t.true(a.equals(b));
  t.false(a.equals(c));
});

test('SrpInteger.xor', (t) => {
  const a = SrpInteger.fromHex('ff');
  const b = SrpInteger.fromHex('0f');
  const result = a.xor(b);
  t.is(result.toHex(), 'f0');
});

test('SrpInteger.randomInteger', (t) => {
  const rand = SrpInteger.randomInteger(16);
  t.is(rand.toHex().length, 32); // 16 bytes = 32 hex chars
});

test('SrpInteger.inspect', (t) => {
  const hex = '123456789abcdef';
  const integer = SrpInteger.fromHex(hex);
  t.is(integer.inspect, `<SRPInteger ${hex}>`);
});