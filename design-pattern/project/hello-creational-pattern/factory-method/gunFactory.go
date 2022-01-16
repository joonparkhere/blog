package factory_method

func getGun(gunType string) iGun {
	if gunType == "ak47" {
		return newAk47()
	}

	if gunType == "musket" {
		return newMusket()
	}

	return nil
}
