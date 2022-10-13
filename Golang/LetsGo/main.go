package main

import "fmt"

func main() {
	fmt.Println("================= Go ==================")
	variables()
	autotest()
}

func variables() {
	var user1 string
	var userAge1 int32
	var userBalance1 float32
	user1, userAge1, userBalance1 = "Husnain", 22, 5003.23
	fmt.Printf("the user %v aged of %v have the balance of %v \n",
		user1, userAge1, userBalance1)

	user2 := "Atif"
	userAge2 := 22
	userBalance2 := 5003.23
	fmt.Printf("the user %v aged of %v have the balance of %v \n",
		user2, userAge2, userBalance2)
	fmt.Printf("%t , %t, %t \n", user2, userAge2, userBalance2)

	const user3 = "Jhon H"
	//	user3 = "Shang Ji"
	fmt.Printf("Hello, %v \n", user3)

	s1 := "go man, Lets go"
	s2 := []rune(s1)
	fmt.Println(s2)
	s2[0] = 'G'
	s3 := string(s2)
	fmt.Println(s3)

	// some string operations
	s4 := "Menk"
	fmt.Printf("character in s1 is %v \n", len(s4))
	s5 := "Mufti"
	blank := " "
	s6 := s4 + blank + s5
	fmt.Println(s6)
	fmt.Printf("first 3 char of s6 is %v \n", s6[:3])

	fmt.Printf("%v \n", s4 > s5)

}

func autotest() {
	autot := "yes";
	fmt.Printf("%v ", autot)
}











