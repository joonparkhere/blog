---
title: 우아한 타입스크립트 요약 1부
date: 2020-11-14
pin: false
tags:
- TS
---

> 목표
> TypeScript로 코딩을 잘하면, 런타입 전에 미리 알 수 있는 오류도 있다.

### 작성자와 사용자의 관점

`타입 시스템`

* 컴파일러에게 사용하는 타입을 명시적으로 지정하는 시스템
* 컴파일러가 자동으로 타입을 추론하는 시스템

`TypeScript의 타입 시스텝`
* 타입을 명시적으로 지정 가능
* 타입을 명시적으로 지정하지 않으면, 타입스크립트 컴파일러가 자동으로 타입을 추론

![](https://images.velog.io/images/tmdgh0221/post/a192b5b7-d37d-4d19-bce2-bc481ac315ce/image.png)

```ts
// 이 함수의 작성자는 매개변수 a 가 number 타입이라는 가정으로 함수를 작성했습니다.
// a 의 타입을 명시적으로 지정하지 않은 경우이기 때문에 a 는 any 로 추론됩니다.
function f(a) {
  return a * 38;
}

// 사용자는 사용법을 숙지하지 않은 채, 문자열을 사용하여 함수를 실행했습니다.
console.log(f(10)); // 380
console.log(f('Mark')); // NaN
```

`noImplicitAny 옵션`
타입을 명시적으로 지정하지 않은 경우, 타입스크립트가 추론 중 `any` 라고 판단하게 되면, 컴파일 에러를 발생시켜 명시적으로 지정하도록 유도합니다.

```ts
// 매개변수의 타입은 명시적으로 지정했습니다.
// 명시적으로 지정하지 않은 함수의 리턴 타입은 number 로 추론됩니다.
function f4(a: number) {
  if (a > 0) {
    return a * 38;
  }
}

// 사용자는 타입에 맞춰 함수를 실행했지만, 실제 undefined + 5 가 실행되어 NaN 이 출력됩니다.
console.log(f4(5)); // 190
console.log(f4(-5) + 5); // NaN
```

`strictNullChecks 옵션`
모든 타입에 자동으로 포함되어 있는 `null` 과 `undefined` 를 제거해줍니다.


```ts
// 매개변수의 타입은 명시적으로 지정했습니다.
// 명시적으로 지정하지 않은 함수의 리턴 타입은 number | undefined 로 추론됩니다.
function f4(a: number) {
  if (a > 0) {
    return a * 38;
  }
}

// 해당 함수의 리턴 타입은 number | undefined 이기 때문에,
// 타입에 따르면 이어진 연산을 바로 할 수 없습니다.
console.log(f4(5));
console.log(f4(-5) + 5); // error TS2532: Object is possibly 'undefined'.
```

작성자의 입장에서 리턴 타입이 정해져있다면 그에 맞춰 함수 signature를 구현할 수 있기 때문에, 명시적으로 리턴 타입을 지정하는 걸 권장합니다.

```ts
// 매개변수의 타입과 함수의 리턴 타입을 명시적으로 지정했습니다.
// 실제 함수 구현부의 리턴 타입과 명시적으로 지정한 타입이 일치하지 않아 컴파일 에러가 발생합니다.

// error TS2366: Function lacks ending return statement and return type does not include 'undefined'.
function f5(a: number): number {
  if (a > 0) {
    return a * 38;
  }
}
```

`noImplicitReturns 옵션`
함수 내에서 모든 코드가 값을 리턴하지 않으면, 컴파일 에러를 발생시킵니다.

```ts
// if 가 아닌 경우 return 을 직접 하지 않고 코드가 종료된다.

// error TS7030: Not all code paths return a value.
function f5(a: number) {
  if (a > 0) {
    return a * 38;
  }
}
```

JavaScipt의 경우 매개변수로 object를 받을 때 특별한 제약을 두지 않습니다. 함수의 인자로 전달받은 object에 필요한 property가 없는 경우 `undefined` 혹은 `NaN`이 리턴될 수 있습니다. 이러한 경우를 막기 위해 object literal type을 사용합니다.

```ts
// JavaScript
function f6(a) {
  return `이름은 ${a.name} 이고, 연령대는 ${
    Math.floor(a.age / 10) * 10
  }대 입니다.`;
}

console.log(f6({ name: 'Mark', age: 38 })); // 이름은 Mark 이고, 연령대는 30대 입니다.
console.log(f6('Mark')); // 이름은 undefined 이고, 연령대는 NaN대 입니다.

//object literal type
function f7(a: { name: string; age: number }): string {
  return `이름은 ${a.name} 이고, 연령대는 ${
    Math.floor(a.age / 10) * 10
  }대 입니다.`;
}

console.log(f7({ name: 'Mark', age: 38 })); // 이름은 Mark 이고, 연령대는 30대 입니다.
console.log(f7('Mark')); // error TS2345: Argument of type 'string' is not assignable to parameter of type '{ name: string; age: number; }'.
```

또한 아래와 같이 타입을 작성자가 지정해서 이용할 수도 있습니다.

```ts
interface PersonInterface {
  name: string;
  age: number;
}

type PersonTypeAlias = {
  name: string;
  age: number;
};

function f8(a: PersonInterface): string {
  return `이름은 ${a.name} 이고, 연령대는 ${
    Math.floor(a.age / 10) * 10
  }대 입니다.`;
}

console.log(f8({ name: 'Mark', age: 38 })); // 이름은 Mark 이고, 연령대는 30대 입니다.
console.log(f8('Mark')); // error TS2345: Argument of type 'string' is not assignable to parameter of type 'PersonInterface'.
```

이 코드에서 사용한 interface와 type alias에 대해서 더 알아보도록 하겠습니다.


### interface와 type alias

`structural type system`은 구조가 같으면, 같은 타입이라고 판단합니다.

```ts
interface IPerson {
  name: string;
  age: number;
  speak(): string;
}

type PersonType = {
  name: string;
  age: number;
  speak(): string;
};

let personInterface: IPerson = {} as any;
let personType: PersonType = {} as any;

personInterface = personType;
personType = personInterface;
```

`nominal type system`은 구조가 같아도 이름이 다르면, 다른 타입이라고 판단합니다.

타입스크립트에서는 nominal 방식을 지원하지 않지만 아래와 같이 꼼수를 부려 흉내낼 수 있습니다.

```ts
type PersonID = string & { readonly brand: unique symbol };

function PersonID(id: string): PersonID {
  return id as PersonID;
}

function getPersonById(id: PersonID) {}

getPersonById(PersonID('id-aaaaaa'));
getPersonById('id-aaaaaa'); // error TS2345: Argument of type 'string' is not assignable to parameter of type 'PersonID'. Type 'string' is not assignable to type '{ readonly brand: unique symbol; }'.
```

interface와 type alias 차이점

1. function
```ts
// type alias
type EatType = (food: string) => void;

// interface
interface IEat {
  (food: string): void;
}
```

2. array
```ts
// type alias
type PersonList = string[];

// interface
interface IPersonList {
  [index: number]: string;
}
```

3. intersection
```ts
interface ErrorHandling {
  success: boolean;
  error?: { message: string };
}

interface ArtistsData {
  artists: { name: string }[];
}

// type alias
type ArtistsResponseType = ArtistsData & ErrorHandling;

// interface
interface IArtistsResponse extends ArtistsData, ErrorHandling {}

let art: ArtistsResponseType;
let iar: IArtistsResponse;
```

4. union type
type alias를 interface에서 상속받거나 class에서 구현은 불가능합니다다.
```ts
interface Bird {
  fly(): void;
  layEggs(): void;
}

interface Fish {
  swim(): void;
  layEggs(): void;
}

type PetType = Bird | Fish;

interface IPet extends PetType {} // error TS2312: An interface can only extend an object type or intersection of object types with statically known members.

class Pet implements PetType {} // error TS2422: A class can only implement an object type or intersection of object types with statically known members.
```

5. interace의 Declaration Merging
같은 이름의 interface가 존재한다면 overriding이 아닌 merging이 됩니다. 추가로, type alias의 Declaration Merging은 아예 제공하지 않는 기능입니다.
```ts
interface MergingInterface {
  a: string;
}

interface MergingInterface {
  b: string;
}
```
![](https://images.velog.io/images/tmdgh0221/post/9356314d-8a44-4e41-a4f6-6a8a84ccd76d/image.png)

### 서브타입과 슈퍼타입
```ts
// sub1 타입은 sup1 타입의 서브 타입이다.
// sup1 타입은 sub1 타입의 슈퍼 타입이다.
let sub1: 1 = 1;
let sup1: number = sub1;
sub1 = sup1; // error! Type 'number' is not assignable to type '1'.

// sub2 타입은 sup2 타입의 서브 타입이다.
// sup2 타입은 sub2 타입의 슈퍼 타입이다.
let sub2: number[] = [1];
let sup2: object = sub2;
sub2 = sup2; // error! Type '{}' is missing the following properties from type 'number[]': length, pop, push, concat, and 16 more.

// sub3 타입은 sup3 타입의 서브 타입이다.
// sup3 타입은 sub3 타입의 슈퍼 타입이다.
let sub3: [number, number] = [1, 2];
let sup3: number[] = sub3;
sub3 = sup3; // error! Type 'number[]' is not assignable to type '[number, number]'. Target requires 2 element(s) but source may have fewer.

// sub5 타입은 sup5 타입의 서브 타입이다.
// sup5 타입은 sub5 타입의 슈퍼 타입이다.
let sub5: never = 0 as never;
let sup5: number = sub5;
sub5 = sup5; // error! Type 'number' is not assignable to type 'never'.

class SubAnimal {}
class SubDog extends SubAnimal {
  eat() {}
}

// sub6 타입은 sup6 타입의 서브 타입이다.
// sup6 타입은 sub6 타입의 슈퍼 타입이다.
let sub6: SubDog = new SubDog();
let sup6: SubAnimal = sub6;
sub6 = sup6;
```

`공변성`이란, 같거나 서브 타입인 경우, 할당이 가능함을 뜻합니다.

```ts
// primitive type
let sub7: string = '';
let sup7: string | number = sub7;

// object - 각각의 프로퍼티가 대응하는 프로퍼티와 같거나 서브타입이어야 한다.
let sub8: { a: string; b: number } = { a: '', b: 1 };
let sup8: { a: string | number; b: number } = sub8;

// array - object 와 마찬가지
let sub9: Array<{ a: string; b: number }> = [{ a: '', b: 1 }];
let sup9: Array<{ a: string | number; b: number }> = sub8
```

`반병성`이란, 함수의 매개변수의 티입이 같거나 슈퍼타입인 경우, 할당이 가능함을 뜻합니다.

```ts
class Person {}
class Developer extends Person {
  coding() {}
}
class StartupDeveloper extends Developer {
  burning() {}
}

function tellme(f: (d: Developer) => Developer) {}

// Developer => Developer 에다가 Developer => Developer 를 할당하는 경우
tellme(function dToD(d: Developer): Developer {
  return new Developer();
});

// Developer => Developer 에다가 Person => Developer 를 할당하는 경우
tellme(function pToD(d: Person): Developer {
  return new Developer();
});

// 특수한 케이스
// Developer => Developer 에다가 StartipDeveloper => Developer 를 할당하는 경우
tellme(function sToD(d: StartupDeveloper): Developer {
  return new Developer();
});
```

`strictFunctionTypes 옵션`
함수의 매개변수 타입만 같거나 슈퍼타입인 경우가 아닌 경우, 에러를 통해 경고합니다.

`any`의 경우 서브타입, 슈퍼타입의 경우 문제가 발생할 소지가 있기 때문에 `unknown`을 사용하는 게 좋습니다.

```ts
// any

// 입력은 마음대로,
// 함수 구현이 자유롭게 => 자유가 항상 좋은건 아니다.
function fany(a: any): number | string | void {
  a.toString();

  if (typeof a === 'number') {
    return a * 38;
  } else if (typeof a === 'string') {
    return `Hello ${a}`;
  }
}

console.log(fany(10)); // 380
console.log(fany('Mark')); // Hello Mark
console.log(fany(true)); // undefined

// unknown

// 입력은 마음대로,
// 함수 구현은 문제 없도록
function funknown(a: unknown): number | string | void {
  a.toString(); // error! Object is of type 'unknown'.

  if (typeof a === 'number') {
    return a * 38;
  } else if (typeof a === 'string') {
    return `Hello ${a}`;
  }
}

console.log(funknown(10)); // 380
console.log(funknown('Mark')); // Hello Mark
console.log(funknown(true)); // undefined
```

### 타입 추론 이해하기

`let`과 `const`의 타입 추론

```ts
let a = 'Mark'; // string
const b = 'Mark'; // 'Mark' => literal type

let c = 38; // number
const d = 38; // 38 => literal type

let e = false; // boolean
const f = false; // false => literal type

let g = ['Mark', 'Haeun']; // string[]
const h = ['Mark', 'Haeun']; // string[]

const i = ['Mark', 'Haeun', 'Bokdang'] as const; // readonly ["Mark", "Haeun", "Bokdang"]
```

타입스크립트는 `Best comon type` 방식으로 타입을 추론합니다. 이 방식은 말 그대로 가장 공통적인 타입을 해당 타입으로 추론함을 의미합니다.

```ts
let j = [0, 1, null]; // (number | null)[]
const k = [0, 1, null]; // (number | null)[]

class Animal {}
class Rhino extends Animal {}
class Elephant extends Animal {}
class Snake extends Animal {}

let l = [new Rhino(), new Elephant(), new Snake()]; // (Rhino | Elephant | Snake)[]
const m = [new Rhino(), new Elephant(), new Snake()]; // (Rhino | Elephant | Snake)[]

const n = [new Animal(), new Rhino(), new Elephant(), new Snake()]; // Animal[]
const o: Animal[] = [new Rhino(), new Elephant(), new Snake()]; // Animal[]
```

또 다른 추론 방식으로, `contextual typing`이 있습니다. 이는 위치에 따라 추론 타입이 달라집니다.

```ts
// Parameter 'e' implicitly has an 'any' type.
const click = (e) => {
  e; // any
};

document.addEventListener('click', click);
document.addEventListener('click', (e) => {
  e; // MouseEvent
});
```

### Type Guard로 안전함을 파악하기

1. `typeof` Type Guard
보통 primitive 타입인 경우 사용합니다.

```ts
function getNumber(value: number | string): number {
  value; // number | string
  if (typeof value === 'number') {
    value; // number
	return value;
  }
  value; // string
  return -1;
}
```

2. `instanceof` Type Guard
```ts
class NegativeNumberError extends Error {}

function getNumber(value: number): number | NegativeNumberError {
  if (value < 0) return new NegativeNumberError();

  return value;
}

function main() {
  const num = getNumber(-10);

  if (num instanceof NegativeNumberError) {
    return;
  }

  num; // number
}
```

3. `in` operator Type Guard
object의 property 유무로 판단해야 하는 경우 사용합니다.

```ts
interface Admin {
  id: string;
  role: string:
}

interface User {
  id: string;
  email: string;
}

function redirect(user: Admin | User) {
  if("role" in user) {
    routeToAdminPage(usr.role);
  } else {
    routeToHomePage(usr.email);
  }
}
```

4. literal Type Guard
object의 property가 같고, 타입이 다른 경우 사용합니다.

```ts
interface IMachine {
  type: string;
}

class Car implements IMachine {
  type: 'CAR';
  wheel: number;
}

class Boat implements IMachine {
  type: 'BOAT';
  motor: number;
}

function getWhellOrMotor(machine: Car | Boat): number {
  if (machine.type === 'CAR') {
    return machine.wheel;
  } else {
    return machine.motor;
  }
}
```

5. custome Type Guard
위의 네가지 방법 중 사용할 수 있는 경우가 없는 경우 구현하여 사용합니다.

```ts
function getWhellOrMotor(machine: any): number {
  if (isCar(machine)) {
    return machine.wheel;
  } else if (isBoat(machine)) {
    return machine.motor;
  } else {
    return -1;
  }
}

function isCar(arg: any): arg is Car {
    return arg.type === 'CAR';
}

function isBoat(arg: any): arg is Boat {
    return arg.type === 'BOAT';
}
```

### class를 안전하게 만들기

예상하지 못한 오류를 막기 위해, class 생성 후 해당 class field 값이 지정되어 있지 않은 경우에 대한 처리합니다.

```ts
// v3.9.7
class Square1 {
  area; // error! implicit any
  sideLength; // error! implicit any
}

class Square2 {
  area: number;
  sideLength: number;
}

const square2 = new Square2();
console.log(square2.area); // compile time - number, runtime - undefined
console.log(square2.sideLength); // compile time - number, runtime - undefined
```

`strictPropertyInitialization 옵션`
Class 의 Property 가 생성자 혹은 선언에서 값이 지정되지 않으면, 컴파일 에러를 발생시켜 주의를 줍니다.

```ts
// v3.9.7
class Square2 {
  area: number; // error TS2564: Property 'area' has no initializer and is not definitely assigned in the constructor.
  sideLength: number; // error TS2564: Property 'sideLength' has no initializer and is not definitely assigned in the constructor.
}

// 사용자는 시도조차 할 수 없도록 만듭니다.
const square2 = new Square2();
console.log(square2.area);
console.log(square2.sideLength);

// 선언과 동시에 초기화
class Square3 {
  area: number = 0;
  sideLength: number = 0;
}

// 생성자를 통한 초기화
class Square4 {
  area: number;
  sideLength: number;

  constructor(sideLength: number) {
    this.sideLength = sideLength;
    this.area = sideLength ** 2;
  }
}
```

그러나 최근의 타입스크립트버전, `v4.0.0` 이후부터는 class property 타입 추론의 동작 방식이 달라집니다.

```ts
// v4.0.2
class Square5 {
  area; // 4 부터는 any 가 아니라, 생성자에 의해 추론된다.
  sideLength; // 4 부터는 any 가 아니라, 생성자에 의해 추론된다.

  constructor(sideLength: number) {
    this.sideLength = sideLength;
    this.area = sideLength ** 2;
  }
}

class Square6 {
  sideLength;

  constructor(sideLength: number) {
    if (Math.random()) {
      this.sideLength = sideLength;
    }
    // else에 대한 처리가 없음
  }

  get area() {
    return this.sideLength ** 2; // error! Object is possibly 'undefined'.
  }
}
```

그러나 여전히, 생성자를 벗어난 범위에서 class property를 처리하면 추론되지 않습니다. 따라서 `!`로 타입 의도를 표현해야 합니다.

```ts
// v4.0.2
class Square7 {
  sideLength!: number; // ! 로 의도를 표현해야 한다.

  constructor(sideLength: number) {
    this.initialize(sideLength);
  }

  initialize(sideLength: number) {
    this.sideLength = sideLength;
  }

  get area() {
    return this.sideLength ** 2;
  }
}
```