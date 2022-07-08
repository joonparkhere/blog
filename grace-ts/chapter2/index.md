---
title: 우아한 타입스크립트 세미나 요약 2부
date: 2020-11-15
pin: false
tags:
- TS
---

## 실전 타입스크립트 코드 작성

### Conditional Type 활용

Generic 타입에 따라 다른 container를 사용하도록 구현할 때 사용합니다.

아래의 예시에서는 `T`가 `string`이면 `StringContainer`, `number`이면 `NumberContainer`가 타입이 되도록 구현하고자 합니다.

```ts
interface StringContainer {
  value: string;
  format(): string;
  split(): string[];
}

interface NumberContainer {
  value: number;
  nearestPrime: number;
  round(): number;
}

// 일반적인 예시
type Item1<T> = {
  id: T,
  container: any;
};

const item1: Item1<string> = {
  id: "aaaaaa",
  container: null
};

// Generic 타입에 따라 구분
type Item2<T> = {
  id: T;
  container: T extends string ? StringContainer : NumberContainer;
};

const item2: Item2<string> = {
  id: 'aaaaaa',
  container: null, // Type 'null' is not assignable to type 'StringContainer'.
};

// 조금 더 강력하게 구분
// T가 string 혹은 number가 아니면 사용 불가
type Item3<T> = {
  id: T extends string | number ? T : never;
  container: T extends string
    ? StringContainer
    : T extends number
    ? NumberContainer
    : never;
};

const item3: Item3<boolean> = {
  id: true, // Type 'boolean' is not assignable to type 'never'.
  container: null, // Type 'null' is not assignable to type 'never'.
};
```

이와 유사한 방법으로 ArrayFilter도 구현 가능합니다.

```ts
type ArrayFilter<T> = T extends any[] ? T : never;

type StringsOrNumbers = ArrayFilter<string | number | string[] | number[]>;

// 아래와 같은 순서로 결과가 도출됩니다.
// 1. string | number | string[] | number[]
// 2. never | never | string[] | number[]
// 3. string[] | number[]
```

마찬가지로, Flatten도 구현 가능합니다.

```ts
type Flatten<T> = T extends any[]
  ? T[number]
  : T extends object
  ? T[keyof T]
  : T;

const numbers = [1, 2, 3];
type NumbersArrayFlattened = Flatten<typeof numbers>;
// 1. number[]
// 2. number

const person = {
  name: 'Mark',
  age: 38
};
                             
type SomeObjectFlattened = Flatten<typeof person>;
// 1. keyof T --> "name" | "age"
// 2. T["name" | "age"] --> T["name"] | T["age"] --> string | number

const isMale = true;
type SomeBooleanFlattened = Flatten<typeof isMale>;
// true
```

`infer` 키워드를 알아보도록 하겠습니다.

```ts
// 보통 Promise의 <> 부분에는 non-error return type이 들어갑니다.
type UnpackPromise<T> = T extends Promise<infer K>[] ? K : any;
// 첫번째 요소는 string, 두번째 요소는 number이다.
const promises = [Promise.resolve('Mark'), Promise.resolve(38)];

type Expected = UnpackPromise<typeof promises>; // string | number
```

이 키워드를 활용해서 함수의 리턴 타입을 알아낼 수 있습니다.

```ts
function plus1(seed: number): number {
  return seed + 1;
}

// <T extends (...args: any) => any>: T가 함수라면 (제약사항)
// 그 함수의 리턴 타입을 infer해서 R로 가져오겠다
type MyReturnType<T extends (...args: any) => any> = T extends (
  ...args: any
) => infer R
  ? R
  : any;

type Id = MyReturnType<typeof plus1>;	// number

lookupEntity(plus1(10));

function lookupEntity(id: Id) {
  // query DB for entity by ID
}
```

타입스크립트에는 내장된 helper conditional type이 있습니다.

```ts
// type Exclude<T, U> = T extends U ? never : T;
type Excluded = Exclude<string | number, string>; // number - diff 역할

// type Extract<T, U> = T extends U ? T : never;
type Extracted = Extract<string | number, string>; // string - filter 역할

// Pick<T, Exclude<keyof T, K>>; (Mapped Type)
type Picked = Pick<{name: string, age: number}, 'name'>;

// type Omit<T, K extends keyof any> = Pick<T, Exclude<keyof T, K>>;
type Omited = Omit<{name: string, age: number}, 'name'>;

// type NonNullable<T> = T extends null | undefined ? never : T;
type NonNullabled = NonNullable<string | number | null | undefined>;

// function의 리턴타입을 가져오기
type ReturnType<T extends (...args: any) => any> = T extends (
  ...args: any
) => infer R
  ? R
  : any;

// function의 매개변수 타입을 가져오기
type Parameters<T extends (...args: any) => any> = T extends (
  ...args: infer P
) => any
  ? P
  : never;

type MyParameters = Parameters<(name: string, age: number) => void>; // [name: string, age: number]

// 예시 생성자
interface Constructor {
  new (name: string, age: number): string;
}

// 생성자의 파라미터 타입을 가져오기
type ConstructorParameters<
  T extends new (...args: any) => any
> = T extends new (...args: infer P) => any ? P : never;

type MyConstructorParameters = ConstructorParameters<Constructor>; // [name: string, age: number]

// 생성자를 통해 만들어진 객체의 타입을 가져오기
type InstanceType<T extends new (...args: any) => any> = T extends new (
  ...args: any
) => infer R
  ? R
  : any;

type MyInstanceType = InstanceType<Constructor>; // string

// function인 property 찾기
type FunctionPropertyNames<T> = {
  [K in keyof T]: T[K] extends Function ? K : never;
}[keyof T];
type FunctionProperties<T> = Pick<T, FunctionPropertyNames<T>>;

// function이 아닌 property 찾기
type NonFunctionPropertyNames<T> = {
  [K in keyof T]: T[K] extends Function ? never : K;
}[keyof T];
type NonFunctionProperties<T> = Pick<T, NonFunctionPropertyNames<T>>;

interface Person {
  id: number;
  name: string;
  hello(message: string): void;
}

type T1 = FunctionPropertyNames<Person>;
type T2 = NonFunctionPropertyNames<Person>;
type T3 = FunctionProperties<Person>;
type T4 = NonFunctionProperties<Person>;
```

### Overloading 활용

먼저 오버로딩이 불가능한 자바스크립트에 타입을 붙이는 경우를 살펴보겠습니다.

```ts
function shuffle(value: string | any[]): string | any[] {
  if (typeof value === 'string')
    return value
      .split('')
      .sort(() => Math.random() - 0.5)
      .join('');
  return value.sort(() => Math.random() - 0.5);
}

console.log(shuffle('Hello, Mark!')); // string | any[]
console.log(shuffle(['Hello', 'Mark', 'long', 'time', 'no', 'see'])); // string | any[]
console.log(shuffle([1, 2, 3, 4, 5])); // string | any[]
```

위의 예시를 제네릭을 사용한 conditional type으로 수정할 수 있습니다.

```ts
function shuffle2<T extends string | any[]>(
  value: T,
): T extends string ? string : T;
function shuffle2(value: any) {
  if (typeof value === 'string')
    return value
      .split('')
      .sort(() => Math.random() - 0.5)
      .join('');
  return value.sort(() => Math.random() - 0.5);
}

// function shuffle2<"Hello, Mark!">(value: "Hello, Mark!"): string
shuffle2('Hello, Mark!');

// function shuffle2<string[]>(value: string[]): string[]
shuffle2(['Hello', 'Mark', 'long', 'time', 'no', 'see']);

// function shuffle2<number[]>(value: number[]): number[]
shuffle2([1, 2, 3, 4, 5]);

// error! Argument of type 'number' is not assignable to parameter of type 'string | any[]'.
shuffle2(1);
```

이외에도 타입스크립트의 오버로딩을 활용할 수도 있습니다.

```ts
function shuffle3(value: string): string;
function shuffle3<T>(value: T[]): T[];
// function 구현 부의 signature는 큰 의미가 없게 된다
function shuffle3(value: string | any[]): string | any[] {
  if (typeof value === 'string')
    return value
      .split('')
      .sort(() => Math.random() - 0.5)
      .join('');
  return value.sort(() => Math.random() - 0.5);
}

shuffle3('Hello, Mark!');
shuffle3(['Hello', 'Mark', 'long', 'time', 'no', 'see']);
shuffle3([1, 2, 3, 4, 5]);
```

추가 예시로, class 내의 method overloading을 살펴보겠습니다.

```ts
class ExportLibraryModal {
  
  public openComponentsToLibrary(
    libraryId: string,
    componentIds: string[],
  ): void;
  public openComponentsToLibrary(componentIds: string[]): void;
  
  // 실제 구현부
  public openComponentsToLibrary(
    libraryIdOrComponentIds: string | string[],
    componentIds?: string[],
  ): void {
    if (typeof libraryIdOrComponentIds === 'string') {
      if (componentIds !== undefined) { // 이건 좀 별루지만,
        // 첫번째 시그니처
        libraryIdOrComponentIds;
        componentIds;
      }
    }

    if (componentIds === undefined) { // 이건 좀 별루지만,
      // 두번째 시그니처
      libraryIdOrComponentIds;
    }
  }
  
}

const modal = new ExportLibraryModal();

modal.openComponentsToLibrary(
  'library-id',
  ['component-id-1', 'component-id-1'],
);

modal.openComponentsToLibrary(['component-id-1', 'component-id-1']);
```

### readonly, as const 남발

`ReadonlyArray<T>`와 `as const`를 사용한 예시를 보겠습니다.

```ts
const weekdays1: ReadonlyArray<string> = [
  'Sunday',
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday',
];

weekdays1[0]; // readonly string[]
weekdays1[0] = 'Fancyday'; // error! Index signature in type 'readonly string[]' only permits reading.

const weekdays2 = [
  'Sunday',
  'Monday',
  'Tuesday',
  'Wednesday',
  'Thursday',
  'Friday',
  'Saturday',
] as const;

weekdays2[0]; // "Sunday"
weekdays2[0] = 'Fancyday'; // error! Cannot assign to '0' because it is a read-only property.
```

내장된 Mapped Types는 어떤게 있는 지 살펴보겠습니다.

```ts
// Make all properties in T optional
type Partial<T> = {
    [P in keyof T]?: T[P];
};

// Make all properties in T required
type Required<T> = {
    [P in keyof T]-?: T[P];
};

// Make all properties in T readonly
type Readonly<T> = {
    readonly [P in keyof T]: T[P];
};

// From T, pick a set of properties whose keys are in the union K
type Pick<T, K extends keyof T> = {
    [P in K]: T[P];
};

// Construct a type with a set of properties K of type T
type Record<K extends keyof any, T> = {
    [P in K]: T;
};
```

이중에서도 Readonly를 사용한 예시를 추가로 살펴보겠습니다.

```ts
interface Book {
  title: string;
  author: string;
}

interface IRootState {
  book: {
    books: Book[];
    loading: boolean;
    error: Error | null;
  };
}

type IReadonlyRootState = Readonly<IRootState>;
let state1: IReadonlyRootState = {} as IReadonlyRootState;
const book1 = state1.book.books[0];
book1.title = 'new';	// 깊게 depth를 파고들지 않기 때문에 변경 가능
```

위의 예시에서 나온 상황처럼, 때로는 DeepReadonly가 필요한 경우가 있을 수 있습니다. 아래와 같이 DeepReadonly를 구현할 수 있습니다.

```ts
// T가 어떤 배열이라면 E는 해당 배열의 요소들이 되는데,
// E에 타고타고 들어가서 모든 요소들을 readonly로 설정
type DeepReadonly<T> = T extends (infer E)[]
  ? ReadonlyArray<DeepReadonlyObject<E>>
  : T extends object
  ? DeepReadonlyObject<T>
  : T;

type DeepReadonlyObject<T> = { readonly [K in keyof T]: DeepReadonly<T[K]> };

type IDeepReadonlyRootState = DeepReadonly<IRootState>;
let state2: IDeepReadonlyRootState = {} as IDeepReadonlyRootState;
const book2 = state2.book.books[0];
book2.title = 'new'; // error! Cannot assign to 'title' because it is a read-only property.
```

### optional type 보다는 Union Type 사용

```ts
type Result1<T> = {
  data?: T;
  error?: Error;
  loading: boolean;
};

declare function getResult1(): Result1<string>;

const r1 = getResult1();
r1.data; // string | undefined
r1.error; // Error | undefined
r1.loading; // boolean

if (r1.data) {
  r1.error; // Error | undefined
  r1.loading; // boolean
}
```

optional type인 경우 위의 `data`와 `error`같이 상충되는 변수들을 효과적으로 구현할 수 없습니다. 따라서 대신에 union type을 활용합니다.

```ts
type Result2<T> =
  | { loading: true }
  | { data: T; loading: false }
  | { error: Error; loading: false };

declare function getResult2(): Result2<string>;

const r2 = getResult2();

r2.data; // error! Property 'data' does not exist on type 'Result2<string>'. Property 'data' does not exist on type '{ loading: true; }'.
r2.error; // error! Property 'error' does not exist on type 'Result2<string>'. Property 'error' does not exist on type '{ loading: true; }'.
r2.loading; // boolean

if ('data' in r2) {
  r2.error; // error! Property 'error' does not exist on type '{ data: string; loading: false; }'.
  r2.loading; // false
}

// 또 다른 guard를 사용한 방법
type Result3<T> =
  | { type: 'pending'; loading: true }
  | { type: 'success'; data: T; loading: false }
  | { type: 'fail'; error: Error; loading: false };

declare function getResult3(): Result3<string>;

const r3 = getResult3();

if (r3.type === 'success') {
  r3; // { type: 'success'; data: string; loading: false; }
}
if (r3.type === 'pending') {
  r3; // { type: 'pending'; loading: true; }
}
if (r3.type === 'fail') {
  r3; // { type: 'fail'; error: Error; loading: false; }
}
```

### never 활용

바로 예시를 통해 확인해보겠습니다.

```ts
enum ToastType {
    AFTER_SAVED,
    AFTER_PUBLISHED,
    AFTER_RESTORE,
}

interface Toast {
    type: ToastType,
    createdAt: string,
}

const toasts: Toast[] = [...];

// if, else if, else로 작동하는 추론
// 잘 동작하지만 toast type이 추가되는 경우에는
// 올바르게 작동하지 안흘 수 있다
// toastNodes2 -> JSX.Element[]
const toastNodes2 = toasts.map((toast) => {
  if (toast.type === ToastType.AFTER_SAVED)
    return (
      <div key={toast.createdAt}>
        <AfterSavedToast />
      </div>
    );
  else if (toast.type === ToastType.AFTER_PUBLISHED)
    return (
      <div key={toast.createdAt}>
        <AfterPublishedToast />
      </div>
    );
  else
    return (
      <div key={toast.createdAt}>
        <AfterRestoredToast />
      </div>
    );
});

// 새로운 toast type이 추가된 경우
// never로 예외 처리할 수 있다
// toastNodes3 -> JSX.Element[]
const toastNodes3 = toasts.map((toast) => {
  if (toast.type === ToastType.AFTER_SAVED)
    return (
      <div key={toast.createdAt}>
        <AfterSavedToast />
      </div>
    );
  else if (toast.type === ToastType.AFTER_PUBLISHED)
    return (
      <div key={toast.createdAt}>
        <AfterPublishedToast />
      </div>
    );
  else if (toast.type === ToastType.AFTER_RESTORE)
    return (
      <div key={toast.createdAt}>
        <AfterRestoredToast />
      </div>
    );
  else return neverExpected(toast.typs);
});

function neverExpected(value: never): never {
  throw new Error(`Unexpected value : ${value}`);
}
```

이상 2020년 08월에 진행된 우아한 테크 세미나의 요약본이었습니다!