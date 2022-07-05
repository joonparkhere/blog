## Go언어에는 어떤 ORM ..

이전에는 주로 스프링을 써서 서버 구현을 하다보니, 아무래도 스프링 데이터 JPA 만큼의 역할을 하는 ORM 라이브러리를 찾고자 한다. 여기서 그 만큼의 역할을 대략 나열해보면 아래와 같다.

- Go언어임을 감안하더라도, 지나친 Low-Level 인터페이스가 아니어야 한다. (쿼리를 Go언어 코드로 작성할 수 있어야 함)
- 깃허브의 스타 수가 1000개 정도는 넘어야 하고, 비교적 최근에도 커밋이 된 기록이 있어야 한다.
- Type-Safe 해야 한다. (런타임 에러를 방지하고자 함)
- Domain 모델 코드를 더럽히지 말아야 한다. (확장성에 대해 열려있기 위함)

다음과 같은 사이트에서 대략적인 라이브러리들을 찾아볼 수 있고, 이외에도 직접 깃허브에서 검색해서 찾아볼 수도 있다.

- [Billo Park님 블로그](https://blog.billo.io/devposts/go_orm_recommandation/)
- [awesome-go-orms 깃허브](https://github.com/d-tsuji/awesome-go-orms)

그 중에서도 Go언어에서 가장 인기 있는 ORM 라이브러리인 GORM과 엔티티를 위한 기능에 최적화된 ENT 라이브러리를 살펴볼 계획이다.

---

## Gorm 빠르게 훑어보기

[GORM](https://github.com/go-gorm/gorm)는 Go언어에서 가장 많이 사용되는 ORM 라이브러리이다. 이름부터 근본 아우라가 느껴진다. 공식 문서에서 아래의 항목들을 강점으로 내세우고 있다.

- Full-Featured ORM 지향
- `Has Many`, `Belongs To`, `Many To Many`, `Polymorphism` 등 연관 관계 지원
- `Before`, `After Create`, `Find`, `Update` 등의 훅 지원
- 즉시 (Eager) 로딩 지원
- SQL 빌더, 서브 쿼리 지원

간단하게나마 사용해보니, 기본적인 ORM 기능들은 모두 지원하는 것 같다. 이 라이브러리와 목적이 유사한 XORM 라이브러리도 있지만 독스가 잘 구성되어 있지않고, 주로 중국어로 설명된 자료들이 있어서 배제했다.

### 세팅

먼저 의존성을 다운 받는다.

```
go get -u gorm.io/gorm
go get gorm.io/driver/mysql
```

### Entity 생성

`User` 엔티티를 Go언어 구조체 형태로 만든다.

```go
type User struct {
	gorm.Model
	Age  int
	Name string
}
```

- `gorm.Model`은 `ID`, `CreatedTime` 등 필드를 갖는다. ([공식 문서](https://gorm.io/docs/models.html))

그리고 MySQL 연결 후 마이그레이트한다.

```go
func main() {
	dsn := "root:1234@tcp(127.0.0.1:3306)/gormDB?charset=utf8mb4&parseTime=True&loc=Local"
	client, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		panic(err)
	}

	if err := client.AutoMigrate(&User{}); err != nil {
		panic(err)
	}
}
```

MySQL DB를 확인해보면 앞서 정의해놓은 구조체에 맞게 `User` 테이블이 생성된다.

```
User Table
----------------------------------------------------------
| id | created_at | updated_at | deleted_at | age | name |
----------------------------------------------------------
```

### CRU

간단한 Create / Read / Update 부분이다.

```go
func main() {
	dsn := "root:1234@tcp(127.0.0.1:3306)/gormDB?charset=utf8mb4&parseTime=True&loc=Local"
	client, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		panic(err)
	}

	if err := client.AutoMigrate(&User{}); err != nil {
		panic(err)
	}

	newUser, err := CreateUser(client)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Create User: %d - %s\n", newUser.Age, newUser.Name)

	readUser, err := ReadUser(client)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Read User: %d - %s\n", readUser.Age, readUser.Name)

	updateUser, err := UpdateUser(client, readUser.ID)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Update User: %d - %s\n", updateUser.Age, updateUser.Name)
}

func CreateUser(client *gorm.DB) (User, error) {
	joon := User{
		Age:  20,
		Name: "joon",
	}

	result := client.Create(&joon)
	return joon, result.Error
}

func ReadUser(client *gorm.DB) (User, error) {
	var joon User

	result := client.Where(&User{
		Name: "joon",
	}).Take(&joon)

	return joon, result.Error
}

func UpdateUser(client *gorm.DB, id uint) (User, error) {
	var joon User
	client.Find(&joon, id)

	joon.Age = 40
	result := client.Save(&joon)

	return joon, result.Error
}
```

이외에도 `[]map[string]interface{}{}` 타입을 파라미터로 받아 쿼리를 날리거나, Batch 트랜잭션 등을 지원한다. [공식 문서](https://gorm.io/docs/create.html)에서 확인할 수 있다.

### 연관 관계 설정

`User`와 1:N 관계를 갖는 `CreditCard`를 설정하려 한다.

```go
type User struct {
	gorm.Model
	Age         int
	Name        string
	CreditCards []CreditCard
}
```

```go
type CreditCard struct {
	gorm.Model
	Number string
	UserID uint
}
```

이어서 데이터를 넣어주고 가져오는 부분이다.

```go
func main() {
	dsn := "root:1234@tcp(127.0.0.1:3306)/gormDB?charset=utf8mb4&parseTime=True&loc=Local"
	client, err := gorm.Open(mysql.Open(dsn), &gorm.Config{})
	if err != nil {
		panic(err)
	}

	if err := client.AutoMigrate(
		&User{},
		&CreditCard{},
	); err != nil {
		panic(err)
	}

	newUser, err := CreateUserWithCreditCard(client)
	if err != nil {
		panic(err)
	}

	err = FindAssociation(client, newUser)
	if err != nil {
		panic(err)
	}
}

func CreateUserWithCreditCard(client *gorm.DB) (User, error) {
	joon := User{
		Age:  20,
		Name: "joon",
		CreditCards: []CreditCard{
			{Number: "1234-1234-1234-1234"},
			{Number: "5678-5678-5678-5678"},
		},
	}

	result := client.Create(&joon)
	return joon, result.Error
}

func FindAssociation(client *gorm.DB, user User) error {
	creditCards := make([]CreditCard, 0)
	err := client.Model(&user).
		Association("CreditCards").
		Find(&creditCards)
	if err != nil {
		return err
	}

	for _, creditCard := range creditCards {
		fmt.Printf("Association Credit Card: %s own %d\n", creditCard.Number, creditCard.UserID)
	}
	return nil
}
```

---

## Ent 빠르게 훑어보기

[ent](https://github.com/ent/ent)는 Go언어의 Entity Framework이다. 공식 문서에서 아래의 항목들을 강점으로 내세우고 있다.

- 그래프 구조로 DB 스키마를 손쉽게 모델링할 수 있다.
- Go언어의 코드로 스키마를 정의할 수 있다.
- 정적인 타입 체킹이 가능하다.
- DB 쿼리와 그래프 탐색를 지원하며 간단하다.

간단하게나마 써본 결과, 자바 진영의 스프링 데이터 JPA 보다는 QueryDSL과 더 잘 매칭된다고 느꼈다. No-Magic으로 유저가 Go 코드로 작성한 Domain 스키마를 토대로 정적인 코드를 생성한 후, 생성된 코드로 쿼리를 작성할 수 있다. 더불어 효과적인 그래프 탐색을 타깃으로 하는 점도 하나의 이유이다. 아래부터는 끄적여본 코드들이다.

### 세팅

먼저 의존성을 다운 받는다.

```ba
go get entgo.io/ent/cmd/ent
go get github.com/go-sql-driver/mysql
```

그리고 프로젝트 디렉토리의 루트에서 아래의 명령어를 치면 `<project>/ent/schema` 위치에 스키마를 위한 큰 틀이 생성된다.

```
go run entgo.io/ent/cmd/ent init User
```

```go
// User holds the schema definition for the User entity.
type User struct {
	ent.Schema
}

// Fields of the User.
func (User) Fields() []ent.Field {
	return nil
}

// Edges of the User.
func (User) Edges() []ent.Edge {
	return nil
}
```

### Entity 생성

`User`에 2개의 필드를 추가해보자.

```go
// Fields of the User.
func (User) Fields() []ent.Field {
	return []ent.Field{
		field.Int("age").
			Positive(),
		field.String("name").
			Default("unknown"),
	}
}
```

`Positive()`, `Default()`와 같이 제약 조건을 걸 수 있고 이외에도 검증 조건 등 옵션이 있다. 자세한 사항은 [공식 독스](https://entgo.io/docs/schema-fields/)에서 확인할 수 있다. 그리고 아래의 명령어를 치면 필요한 코드 파일들을 생성해준다.

```
go generate ./ent
// ent generate --idtype int64 --feature sql/upsert ./ent/schema
```

```
ent
├── client.go
├── config.go
├── context.go
├── ent.go
├── generate.go
├── mutation.go
... truncated
├── schema
│   └── user.go
├── tx.go
├── user
│   ├── user.go
│   └── where.go
├── user.go
├── user_create.go
├── user_delete.go
├── user_query.go
└── user_update.go
```

이처럼 꽤나 많은 파일이 생성되며 스키마 관련 파일, DB 쿼리 메서드를 위한 파일 외에도 DB 커넥션과 같은 용도를 위한 파일도 같이 생성된다. 이어서 DB에 엔티티를 생성해보자.

```go
import (
	_ "github.com/go-sql-driver/mysql"
)

func main() {
	client, err := ent.Open(dialect.MySQL, "root:1234@tcp(127.0.0.1:3306)/entDB?parseTime=True")
	if err != nil {
		panic(err)
	}
	defer client.Close()

	if err = client.Schema.Create(context.Background()); err != nil {
		panic(err)
	}
}
```

```
User Table
-------------------
| id | age | name |
-------------------
```

- MySQL을 DB로 사용했는데, 해당 DB 드라이버를 직접 import해줘야 오류가 안나고 정상 동작한다.

### CRU

아래는 간단한 C/R/U 코드이다.

```GO
func CreateUser(ctx context.Context, client *ent.Client) (*ent.User, error) {
	newUser, err := client.User.
		Create().
		SetAge(25).
		SetName("joonpark").
		Save(ctx)
	if err != nil {
		return nil, err
	}

	return newUser, nil
}
```

```go
func ReadUser(ctx context.Context, client *ent.Client) (*ent.User, error) {
	findUser, err := client.User.
		Query().
		Where(user.Name("joonpark")).
		Only(ctx)
	if err != nil {
		return nil, err
	}

	return findUser, nil
}
```

```go
func UpdateUser(ctx context.Context, client *ent.Client, id int) (*ent.User, error) {
	updateUser, err := client.User.
		UpdateOneID(id).
		SetAge(20).
		Save(ctx)
	if err != nil {
		return nil, err
	}
	
	return updateUser, nil
}
```

```go
func main() {
	client, err := ent.Open(dialect.MySQL, "root:1234@tcp(127.0.0.1:3306)/entDB?parseTime=True")
	if err != nil {
		panic(err)
	}
	defer client.Close()

	ctx := context.Background()

	if err = client.Schema.Create(ctx); err != nil {
		panic(err)
	}

	if _, err := CreateUser(ctx, client); err != nil {
		panic(err)
	}

	findUser, err := ReadUser(ctx, client)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Read User age, name: %d, %s\n", findUser.Age, findUser.Name)

	updateUser, err := UpdateUser(ctx, client, findUser.ID)
	if err != nil {
		panic(err)
	}
	fmt.Printf("Update User age, name: %d, %s\n", updateUser.Age, updateUser.Name)
}
```

조금 더 자세한 예제 코드는 [공식 문서](https://entgo.io/docs/crud)에서 찾아볼 수 있다.

### 연관 관계 설정

`User`가 `Car`와는 1:N 관계를, `Group`와는 M:N 관계를 갖는 엔티티를 구성하려 한다. 우선 마찬가지로 각 엔티티의 초기 세팅을 하고, 필드들을 추가한다. 먼저 `Car` 엔티티를 중점으로 설명하고 `Group` 엔티티를 얘기하자.

```
go run entgo.io/ent/cmd/ent init Car Group
```

![`User`와 `Car` 연관 관계](images/ent_re_cars_owner.png)

```go
// Car holds the schema definition for the Car entity.
type Car struct {
	ent.Schema
}

// Fields of the Car.
func (Car) Fields() []ent.Field {
	return []ent.Field{
		field.String("model"),
		field.Time("registered_at"),
	}
}
```

그리고 `User` 엔티티에 연관 관계 (Edge라고 칭함) 를 추가해준다.

```go
// Edges of the User.
func (User) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("cars", Car.Type),
	}
}
```

이어서 `Car` 엔티티에 연관 관계 (Back-Edge) 를 추가해준다.

```go
// Edges of the Car.
func (Car) Edges() []ent.Edge {
	return []ent.Edge{
		edge.From("owner", User.Type).
			Ref("cars").
			Unique(),
	}
}
```

이후 아래와 같이 사용할 수 있다.

```go
func main() {
	client, err := ent.Open(dialect.MySQL, "root:1234@tcp(127.0.0.1:3306)/entDB?parseTime=True")
	if err != nil {
		panic(err)
	}
	defer client.Close()

	ctx := context.Background()

	if err := client.Schema.Create(ctx); err != nil {
		panic(err)
	}

	newUser, err := CreateUserWithCars(ctx, client)
	if err != nil {
		panic(err)
	}

	newUserCars, err := newUser.QueryCars().All(ctx)
	if err != nil {
		panic(err)
	}
	for _, userCar := range newUserCars {
		fmt.Printf("New User Car: %s (%s)\n", userCar.Model, userCar.RegisteredAt)
	}
}

func CreateUserWithCars(ctx context.Context, client *ent.Client) (*ent.User, error) {
	tesla, err := client.Car.
		Create().
		SetModel("Tesla").
		SetRegisteredAt(time.Now()).
		Save(ctx)
	if err != nil {
		return nil, err
	}

	ford, err := client.Car.
		Create().
		SetModel("Ford").
		SetRegisteredAt(time.Now()).
		Save(ctx)
	if err != nil {
		return nil, err
	}

	joon, err := client.User.
		Create().
		SetAge(15).
		SetName("joon-edge").
		AddCars(tesla, ford).
		Save(ctx)
	if err != nil {
		return nil, err
	}

	return joon, nil
}
```

다음으로 `Group` 엔티티도 빠르게 설정해준다.

![`Group`과 `User` 연관 관계](images/ent_re_group_users.png)

```go
// Group holds the schema definition for the Group entity.
type Group struct {
	ent.Schema
}

// Fields of the Group.
func (Group) Fields() []ent.Field {
	return []ent.Field{
		field.String("name").
			Match(regexp.MustCompile("[a-zA-Z_]+$")),
	}
}

// Edges of the Group.
func (Group) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("users", User.Type),
	}
}
```

```go
// Edges of the User.
func (User) Edges() []ent.Edge {
	return []ent.Edge{
		edge.To("cars", Car.Type),
		edge.From("groups", Group.Type).
			Ref("users"),
	}
}
```

### 그래프 탐색

먼저 필요한 데이터를 넣어준다.

```go
func CreateGraph(ctx context.Context, client *ent.Client) error {
	joon, err := client.User.
		Create().
		SetAge(30).
		SetName("joon").
		Save(ctx)
	if err != nil {
		return err
	}
	park, err := client.User.
		Create().
		SetAge(40).
		SetName("park").
		Save(ctx)
	if err != nil {
		return err
	}

	err = client.Car.
		Create().
		SetModel("Tesla").
		SetRegisteredAt(time.Now()).
		SetOwner(joon).
		Exec(ctx)
	if err != nil {
		return err
	}
	err = client.Car.
		Create().
		SetModel("Mazda").
		SetRegisteredAt(time.Now()).
		SetOwner(joon).
		Exec(ctx)
	if err != nil {
		return err
	}
	err = client.Car.
		Create().
		SetModel("Ford").
		SetRegisteredAt(time.Now()).
		SetOwner(park).
		Exec(ctx)
	if err != nil {
		return err
	}

	err = client.Group.
		Create().
		SetName("Netlify").
		AddUsers(park, joon).
		Exec(ctx)
	if err != nil {
		return err
	}
	err = client.Group.
		Create().
		SetName("GitHub").
		AddUsers(joon).
		Exec(ctx)
	if err != nil {
		return err
	}

	return nil
}
```

그리고 다음과 같이 탐색할 수 있다.

```go
func ReadJoonCars(ctx context.Context, client *ent.Client) error {
	joon := client.User.
		Query().
		Where(
			user.HasCars(),
			user.Name("joon"),
		).
		OnlyX(ctx)

	joonCars, err := joon.
		QueryGroups().
		QueryUsers().
		QueryCars().
		Where(
			car.Not(
				car.Model("Mazda"),
			),
		).
		All(ctx)
	if err != nil {
		return err
	}

	for _, joonCar := range joonCars {
		fmt.Printf("Joon Own Car - Car: %s\n", joonCar.Model)
	}

	return nil
}
```

---

## Mgm 선택적으로 훑어보기

[mgm](https://github.com/Kamva/mgm)는 몽고DB를 위한 ODM (Object-Document Mapper) 라이브러리다. 생각보다 Go언어에서 몽고DB 드라이버를 지원하는 툴들이 전무하다싶이 해서 선택지가 없었다. 진행 중인 프로젝트에서 몽고DB를 적극적으로 활용하지는 않고, 주로 DB에 있는 Document를 긁어와서, 연관된 값을 갖는 다른 Document를 가져오는 정도로만 사용할 예정이다.

### 세팅

먼저 의존성을 다운 받는다.

```
go get github.com/kamva/mgm/v3
```

### Entity 선언

진행 중인 프로젝트의 경우 이더리움 트랜잭션의 로그에 해당하는 값이 몽고 DB에 간략하게 저장되어 있는 상태이다. 현 DB의 Collection Document 상태에 맞춰 구조체를 선언한다.

```go
type TransferEvent struct {
	mgm.DefaultModel
	Address          string `json:"address" bson:"address"`
	BlockHash        string `json:"block_hash" bson:"block_hash"`
	BlockNumber      int64  `json:"block_number" bson:"block_number"`
	BlockTimestamp   int64  `json:"block_timestamp" bson:"block_timestamp"`
	CreatedTimestamp int64  `json:"created_timestamp" bson:"created_timestamp"`
	FromAddress      string `json:"from_address" bson:"from_address"`
	LogIndex         int64  `json:"log_index" bson:"log_index"`
	MethodID         string `json:"method_id" bson:"method_id"`
	ToAddress        string `json:"to_address" bson:"to_address"`
	TokenID          string `json:"token_id" bson:"token_id"`
	TransactionHash  string `json:"transaction_hash" bson:"transaction_hash"`
	UpdatedTimestamp int64  `json:"updated_timestamp" bson:"updated_timestamp"`
}
```

그리고 몽고DB와 커넥션을 한다.

```go
func main() {
	if err := connect(); err != nil {
		panic(err)
	}
}

func connect() error {
	err := mgm.SetDefaultConfig(nil, "rawDB", options.Client().ApplyURI("mongodb://127.0.0.1:27017"))
	return err
}
```

### Find Document

먼저 Collection을 명시하고, 찾을 Document에 대한 제약 조건을 정의한다.

```go
func main() {
	// ...
	ctx := mgm.Ctx()
	transferCollection := mgm.CollectionByName("eth.selected_event.transfer")
}
```

그리고 현재 몽고DB에는 8만여개의 Document가 담겨있어서 한번의 쿼리로 긁어오려고 하면 에러가 발생한다. 따라서 우선적으로 Limit을 설정해서 찾도록 했다. 향후에는 고루틴을 사용해서 병렬적으로 처리하도록 구성할 생각이다.

```go
func main() {
	// ...

	countDocuments, err := transferCollection.CountDocuments(ctx, bson.M{})
	if err != nil {
		panic(err)
	}
	fmt.Printf("document count: %d\n", countDocuments)

	findCursor, err := transferCollection.Find(
		ctx, 
		bson.M{}, 
		options.Find().SetLimit(10), 
		options.Find().SetSkip(0),
	)
	if err != nil {
		panic(err)
	}

	transferEvents := make([]TransferEvent, 0)
	if err = findCursor.All(ctx, &transferEvents); err != nil {
		panic(err)
	}
	for _, transferEvent := range transferEvents {
		unixTime := time.Unix(transferEvent.CreatedTimestamp, 0)
		fmt.Printf("event's tx hash: %s at %s\n", transferEvent.TransactionHash, unixTime)
	}
}
```

이제 찾은 `Transfer Event`의 `transactionHash`와 같은 값을 갖는 Document를 다른 Collection에서 찾아야 한다.

```go
func main() {
	// ...
    
    transferEvents := make([]TransferEvent, 0)
	if err = findCursor.All(ctx, &transferEvents); err != nil {
		panic(err)
	}
	for _, transferEvent := range transferEvents {
		txHash := transferEvent.TransactionHash
		fmt.Printf("event's tx hash: %s\n", txHash)

		lastChar := txHash[len(txHash)-1:]
		rawTxCollectionName := "eth.raw_tx_" + lastChar
		rawTxCollection := mgm.CollectionByName(rawTxCollectionName)

		var rawTransaction RawTransaction
		err := rawTxCollection.First(
			bson.M{"hash": bson.M{
				operator.Eq: txHash,
			}}, 
			&rawTransaction,
		)
		if err != nil {
			panic(err)
		}
		fmt.Printf("matched tx hash: %s\n", rawTransaction.Hash)
	}
}
```

이렇게 몽고DB의 어느 한 Collection에서 Document를 가져온 후, 해당 Document의 특정 값을 기준으로 다른 Collection의 Document를 찾는 과정을 해보았다.

