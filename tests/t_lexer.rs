use plm::lexer::*;

macro_rules! assert_token {
	($lexer: ident, $val: pat) => {
		match $lexer.next() {
		$val => {}
		_ => { assert!(false) }
		}
	};
}

macro_rules! assert_identifier {
	($lexer: ident, $val: literal) => {
		match $lexer.next() {
		Some((Token::Identifier(s), _)) => {
			let outcome = (s.as_str() == $val);
			if !outcome {
				println!("Got \"{}\" when it was supposed to be \"{}\"",
					s, $val);
			}
			assert!(outcome)
		}
		_ => { assert!(false) }
		}
	};
}

#[test]
fn test_initialisation() {
	let mut lexer = Lexer::from_string("".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_special_token0() {
	let mut lexer = Lexer::from_string(";".to_string());
	assert_token!(lexer, Some((Token::SemiColon, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token1() {
	let mut lexer = Lexer::from_string("$".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_special_token2() {
	let mut lexer = Lexer::from_string("=".to_string());
	assert_token!(lexer, Some((Token::Equal, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token3() {
	let mut lexer = Lexer::from_string(".".to_string());
	assert_token!(lexer, Some((Token::Dot, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token4() {
	let mut lexer = Lexer::from_string("/".to_string());
	assert_token!(lexer, Some((Token::Slash, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token5() {
	let mut lexer = Lexer::from_string("(".to_string());
	assert_token!(lexer, Some((Token::LParan, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token6() {
	let mut lexer = Lexer::from_string(")".to_string());
	assert_token!(lexer, Some((Token::RParan, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token7() {
	let mut lexer = Lexer::from_string("+".to_string());
	assert_token!(lexer, Some((Token::Plus, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token8() {
	let mut lexer = Lexer::from_string("-".to_string());
	assert_token!(lexer, Some((Token::Minus, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token9() {
	let mut lexer = Lexer::from_string("'".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_special_token10() {
	let mut lexer = Lexer::from_string("*".to_string());
	assert_token!(lexer, Some((Token::Star, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token11() {
	let mut lexer = Lexer::from_string(",".to_string());
	assert_token!(lexer, Some((Token::Comma, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token12() {
	let mut lexer = Lexer::from_string("<".to_string());
	assert_token!(lexer, Some((Token::Less, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token13() {
	let mut lexer = Lexer::from_string(">".to_string());
	assert_token!(lexer, Some((Token::Greater, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token14() {
	let mut lexer = Lexer::from_string(":".to_string());
	assert_token!(lexer, Some((Token::Colon, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token15() {
	let mut lexer = Lexer::from_string(" ".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_special_token16() {
	let mut lexer = Lexer::from_string("\t".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_special_token17() {
	let mut lexer = Lexer::from_string("\r".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_special_token18() {
	let mut lexer = Lexer::from_string("\n".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_special_token19() {
	let mut lexer = Lexer::from_string("<=".to_string());
	assert_token!(lexer, Some((Token::LessEqual, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token20() {
	let mut lexer = Lexer::from_string(">=".to_string());
	assert_token!(lexer, Some((Token::GreaterEqual, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token21() {
	let mut lexer = Lexer::from_string("<".to_string());
	assert_token!(lexer, Some((Token::Less, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_special_token22() {
	let mut lexer = Lexer::from_string("<>".to_string());
	assert_token!(lexer, Some((Token::NotEqual, _)));
	assert_token!(lexer, None)
}

#[test]
fn test_invalid_number0() {
	let mut lexer = Lexer::from_string("01l,Y".to_string());
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number1() {
	let mut lexer = Lexer::from_string("36:$".to_string());
	assert_token!(lexer, Some((Token::Number(36), _)));
	assert_token!(lexer, Some((Token::Colon, _)));
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number2() {
	let mut lexer = Lexer::from_string("83ew".to_string());
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number3() {
	let mut lexer = Lexer::from_string("30nKn".to_string());
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number4() {
	let mut lexer = Lexer::from_string("38lW".to_string());
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number5() {
	let mut lexer = Lexer::from_string("66PID".to_string());
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number6() {
	let mut lexer = Lexer::from_string("33wQ".to_string());
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number7() {
	let mut lexer = Lexer::from_string("91IC".to_string());
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number8() {
	let mut lexer = Lexer::from_string("23/".to_string());
	assert_token!(lexer, Some((Token::Number(23), _)));
	assert_token!(lexer, Some((Token::Slash, _)));
	assert_token!(lexer, None)
}


#[test]
fn test_invalid_number9() {
	let mut lexer = Lexer::from_string("07Xv".to_string());
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number0() {
	let mut lexer = Lexer::from_string("354356D".to_string());
	assert_token!(lexer, Some((Token::Number(354356), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number1() {
	let mut lexer = Lexer::from_string("10111110100011010B".to_string());
	assert_token!(lexer, Some((Token::Number(97562), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number2() {
	let mut lexer = Lexer::from_string("    202117d".to_string());
	assert_token!(lexer, Some((Token::Number(202117), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number3() {
	let mut lexer = Lexer::from_string("1124013o    ".to_string());
	assert_token!(lexer, Some((Token::Number(305163), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number4() {
	let mut lexer = Lexer::from_string("111000011111110011b".to_string());
	assert_token!(lexer, Some((Token::Number(231411), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number5() {
	let mut lexer = Lexer::from_string("688978D".to_string());
	assert_token!(lexer, Some((Token::Number(688978), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number6() {
	let mut lexer = Lexer::from_string("0ca7f8h".to_string());
	assert_token!(lexer, Some((Token::Number(829432), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number7() {
	let mut lexer = Lexer::from_string("11101110110101000000b".to_string());
	assert_token!(lexer, Some((Token::Number(978240), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number8() {
	let mut lexer = Lexer::from_string("2001241q".to_string());
	assert_token!(lexer, Some((Token::Number(524961), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_number9() {
	let mut lexer = Lexer::from_string("1375475Q".to_string());
	assert_token!(lexer, Some((Token::Number(391997), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier0() {
	let mut lexer = Lexer::from_string("a$".to_string());
	assert_identifier!(lexer, "A");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier1() {
	let mut lexer = Lexer::from_string("plv$kU0Yn0tuTQC3pXNNv$Jnbdb".to_string());
	assert_identifier!(lexer, "PLVKU0YN0TUTQC3PXNNVJNBDB");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier2() {
	let mut lexer = Lexer::from_string("AtP4I1Hi1RgLRcVLgBzXELk8PJwHecc1wzFnt$h11MyvHmBr6s5xOQNOdaY5I0PJQ8ScqT172dCnMqq8BcCYIIfoJRB39K1Lgy7jic2sX    ".to_string());
	assert_identifier!(lexer, "ATP4I1HI1RGLRCVLGBZXELK8PJWHECC1WZFNTH11MYVHMBR6S5XOQNODAY5I0PJQ8SCQT172DCNMQQ8BCCYIIFOJRB39K1LGY7JIC2SX");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier3() {
	let mut lexer = Lexer::from_string("    LDBWARGp4MiulqWezW2dyVrtmJHzVdKulx".to_string());
	assert_identifier!(lexer, "LDBWARGP4MIULQWEZW2DYVRTMJHZVDKULX");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier4() {
	let mut lexer = Lexer::from_string("jYw8qjy78dfIr725oWgd1JWY9S2ptAE680yT5oIh8svx8GmOpoAJyIrEzFPUihzX6OYToZ7Wvd8UJv3rEP5mtd692Qs7dJQIE".to_string());
	assert_identifier!(lexer, "JYW8QJY78DFIR725OWGD1JWY9S2PTAE680YT5OIH8SVX8GMOPOAJYIREZFPUIHZX6OYTOZ7WVD8UJV3REP5MTD692QS7DJQIE");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier5() {
	let mut lexer = Lexer::from_string("IwKgr$aTN6zMSei3WaNHA16ys2wnLELl7QP8T5FNtWM4QxUj6lvwfUEcl0gf$TXHiDXOxC3xlrwFaw4FgU1ue3j3QbTjeBkSX".to_string());
	assert_identifier!(lexer, "IWKGRATN6ZMSEI3WANHA16YS2WNLELL7QP8T5FNTWM4QXUJ6LVWFUECL0GFTXHIDXOXC3XLRWFAW4FGU1UE3J3QBTJEBKSX");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier6() {
	let mut lexer = Lexer::from_string("RuNqbaO".to_string());
	assert_identifier!(lexer, "RUNQBAO");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier7() {
	let mut lexer = Lexer::from_string("tWFBPR8v1hFploLU6z8OvZIXUVfDhh$yYSGUl4alsjMkoEsJS".to_string());
	assert_identifier!(lexer, "TWFBPR8V1HFPLOLU6Z8OVZIXUVFDHHYYSGUL4ALSJMKOESJS");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier8() {
	let mut lexer = Lexer::from_string("    Edn1pijs9gOd1sk6GhUUn5rYaWag2PZdlXHG93gqf2Dy2DfuD8C4aYXGcE0PenvGBxKX9NlRZL7gzXSDl1Vp1w1asFddTh4E3gwTF5JebMX2nZ8kVBUQf9g".to_string());
	assert_identifier!(lexer, "EDN1PIJS9GOD1SK6GHUUN5RYAWAG2PZDLXHG93GQF2DY2DFUD8C4AYXGCE0PENVGBXKX9NLRZL7GZXSDL1VP1W1ASFDDTH4E3GWTF5JEBMX2NZ8KVBUQF9G");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier9() {
	let mut lexer = Lexer::from_string("siS8dbRgE$yi0P47zkW02GyYVbJ5$881ahHBonqwNlD$mMrr5".to_string());
	assert_identifier!(lexer, "SIS8DBRGEYI0P47ZKW02GYYVBJ5881AHHBONQWNLDMMRR5");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier10() {
	let mut lexer = Lexer::from_string("eynneH28hTeJnN6fNeKeollFwnWFhciNgtMrWY2vTcLjYOi13TdpjNMaA2kWX75Ak7giAIWT2Ayd40QZcFBylTjdnYpNZJRpkUJ$d".to_string());
	assert_identifier!(lexer, "EYNNEH28HTEJNN6FNEKEOLLFWNWFHCINGTMRWY2VTCLJYOI13TDPJNMAA2KWX75AK7GIAIWT2AYD40QZCFBYLTJDNYPNZJRPKUJD");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier11() {
	let mut lexer = Lexer::from_string("L5cHqWpT3e31pLqeNdCns1J0DORsJsXQoznV13qyj1hRdW".to_string());
	assert_identifier!(lexer, "L5CHQWPT3E31PLQENDCNS1J0DORSJSXQOZNV13QYJ1HRDW");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier12() {
	let mut lexer = Lexer::from_string("TKcgWxKQ4kNamFo$DCy1AY1OdH5ysTYk4VQVNpL44Yi86J6KIXbyBAmoMsp9TbQBk8JvunOD3KxTD5EOgiqjx9pcj1BQjapjHhC0".to_string());
	assert_identifier!(lexer, "TKCGWXKQ4KNAMFODCY1AY1ODH5YSTYK4VQVNPL44YI86J6KIXBYBAMOMSP9TBQBK8JVUNOD3KXTD5EOGIQJX9PCJ1BQJAPJHHC0");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier13() {
	let mut lexer = Lexer::from_string("Rr5xJO9sE5EaSWDufQYO4M8FIZlNvohah93AwR6qDrvA$rnsMgjuHpq0tJjV74mkxvhFKi23i8Bnqx1UYYRVtPqAcAautl46qHSgvabhVonmj3qK".to_string());
	assert_identifier!(lexer, "RR5XJO9SE5EASWDUFQYO4M8FIZLNVOHAH93AWR6QDRVARNSMGJUHPQ0TJJV74MKXVHFKI23I8BNQX1UYYRVTPQACAAUTL46QHSGVABHVONMJ3QK");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier14() {
	let mut lexer = Lexer::from_string("Txg9Vw1WsW$fa6G2hbQTvA1ljJXB5SgKqSotoO$khMfm2Ig9".to_string());
	assert_identifier!(lexer, "TXG9VW1WSWFA6G2HBQTVA1LJJXB5SGKQSOTOOKHMFM2IG9");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier15() {
	let mut lexer = Lexer::from_string("q2Hfyt6qgxo$E8xnrzEHsN7QPRSpSmX4wG5fhVFEt0JARon01hRvr11BEXCqlTT2bNL1oK0ME$cARZXXqH1koCkEUVG3wX".to_string());
	assert_identifier!(lexer, "Q2HFYT6QGXOE8XNRZEHSN7QPRSPSMX4WG5FHVFET0JARON01HRVR11BEXCQLTT2BNL1OK0MECARZXXQH1KOCKEUVG3WX");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier16() {
	let mut lexer = Lexer::from_string("f5qFXeiaIpgLQg7KSCwqyQH".to_string());
	assert_identifier!(lexer, "F5QFXEIAIPGLQG7KSCWQYQH");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier17() {
	let mut lexer = Lexer::from_string("gFpW$3BGnzTip9s6ymQMr5LK6R2jCvtcZnKTDgOSpP9Zov".to_string());
	assert_identifier!(lexer, "GFPW3BGNZTIP9S6YMQMR5LK6R2JCVTCZNKTDGOSPP9ZOV");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier18() {
	let mut lexer = Lexer::from_string("tfpsgnEoBS5Y4e4laItjRrsgzxaeSE".to_string());
	assert_identifier!(lexer, "TFPSGNEOBS5Y4E4LAITJRRSGZXAESE");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier19() {
	let mut lexer = Lexer::from_string("Pi$0gfqxqnFXnnPNBhOgWBajgAueURpp0JsnbNiCiQPLQq1Q$x2L01o3c1P6VY".to_string());
	assert_identifier!(lexer, "PI0GFQXQNFXNNPNBHOGWBAJGAUEURPP0JSNBNICIQPLQQ1QX2L01O3C1P6VY");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier20() {
	let mut lexer = Lexer::from_string("AMU8uRiD0iAEzXOvPnRq6aScCq2YljjUa3oRT6zodOpCL28Xeh67SZoCPbzGr7f0tbtVBMgXdMowR9XCPBoIxtpfnOe6xZAsRfW2563tmMaDbyam".to_string());
	assert_identifier!(lexer, "AMU8URID0IAEZXOVPNRQ6ASCCQ2YLJJUA3ORT6ZODOPCL28XEH67SZOCPBZGR7F0TBTVBMGXDMOWR9XCPBOIXTPFNOE6XZASRFW2563TMMADBYAM");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier21() {
	let mut lexer = Lexer::from_string("COCHh8lmFdY$QZqCGO$gI17fPyzhEhPDmZTNmepp7ID9".to_string());
	assert_identifier!(lexer, "COCHH8LMFDYQZQCGOGI17FPYZHEHPDMZTNMEPP7ID9");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier22() {
	let mut lexer = Lexer::from_string("XW4uNpfmMXQkRfuybTM0n5f9xksUIKrzehA2xyRx8QMPapjCPfPg47fJWAZzVbN0jdiEIHrOL4kUdXKzClgxusjX4ib5ZcXWiSKHIgGOzwB0".to_string());
	assert_identifier!(lexer, "XW4UNPFMMXQKRFUYBTM0N5F9XKSUIKRZEHA2XYRX8QMPAPJCPFPG47FJWAZZVBN0JDIEIHROL4KUDXKZCLGXUSJX4IB5ZCXWISKHIGGOZWB0");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier23() {
	let mut lexer = Lexer::from_string("M0sazyN8RFTZcEnddo4MJ97Y8R3mzDE3RwQLSAn".to_string());
	assert_identifier!(lexer, "M0SAZYN8RFTZCENDDO4MJ97Y8R3MZDE3RWQLSAN");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier24() {
	let mut lexer = Lexer::from_string("cGKZ1bfEdjOWH0nEeS3y7N6l3idEJY3gRVKMcNv6Jegis6b6vQE0223jz4LvDwLNSF4SXkhjWT70sbIoXB$z".to_string());
	assert_identifier!(lexer, "CGKZ1BFEDJOWH0NEES3Y7N6L3IDEJY3GRVKMCNV6JEGIS6B6VQE0223JZ4LVDWLNSF4SXKHJWT70SBIOXBZ");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier25() {
	let mut lexer = Lexer::from_string("BdfeOOxm6tneK8FI5CSwwW82iwrC40IieFo4sh4bXy99J2kfijyLNInySfZrRusxCly2555hzQPzX1dWN7jfYbinUMvXWVIPbXJEnLT".to_string());
	assert_identifier!(lexer, "BDFEOOXM6TNEK8FI5CSWWW82IWRC40IIEFO4SH4BXY99J2KFIJYLNINYSFZRRUSXCLY2555HZQPZX1DWN7JFYBINUMVXWVIPBXJENLT");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier26() {
	let mut lexer = Lexer::from_string("QI69sbnBep9VGRF37oTxASCb8HJR8tjB5UG1FujK6wUCeGJKlHAVJOKVXRoXbwr3nYLyiSb44hnxj$CosHQuBSeXNA0SupaiT4".to_string());
	assert_identifier!(lexer, "QI69SBNBEP9VGRF37OTXASCB8HJR8TJB5UG1FUJK6WUCEGJKLHAVJOKVXROXBWR3NYLYISB44HNXJCOSHQUBSEXNA0SUPAIT4");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier27() {
	let mut lexer = Lexer::from_string("BtV1s3tYxSksZB$t6VAStW1DtRp1APzDg5aggc4IvXYSPNdUKTDzxLb9AlLYxXe2cPTDDjv4Bssomt1ioexmtdf4ORnjq$DkMTT3o".to_string());
	assert_identifier!(lexer, "BTV1S3TYXSKSZBT6VASTW1DTRP1APZDG5AGGC4IVXYSPNDUKTDZXLB9ALLYXXE2CPTDDJV4BSSOMT1IOEXMTDF4ORNJQDKMTT3O");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier28() {
	let mut lexer = Lexer::from_string("w2XN4CGvZJwG7yPm1rakGELxEtWljoafmtfc4eorLqR8X1oA7Tun6GUgKnEeAbupjtYI4Jmd8oHD8Da8cO".to_string());
	assert_identifier!(lexer, "W2XN4CGVZJWG7YPM1RAKGELXETWLJOAFMTFC4EORLQR8X1OA7TUN6GUGKNEEABUPJTYI4JMD8OHD8DA8CO");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier29() {
	let mut lexer = Lexer::from_string("QaQvwAjuo9UKro9QuiGmc1nAItyWOAPwxG5apOynl".to_string());
	assert_identifier!(lexer, "QAQVWAJUO9UKRO9QUIGMC1NAITYWOAPWXG5APOYNL");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier30() {
	let mut lexer = Lexer::from_string("lVj6YJDQftggckkBZ1hRyJtgWsZFi$MjWlotBA6oRcHDVftlhcRxu5nRKLKs8zdnQORYyMIQlV2EVKZKbQTLbhugbmjOsiCOzGp$Dy2tym2H0rOx".to_string());
	assert_identifier!(lexer, "LVJ6YJDQFTGGCKKBZ1HRYJTGWSZFIMJWLOTBA6ORCHDVFTLHCRXU5NRKLKS8ZDNQORYYMIQLV2EVKZKBQTLBHUGBMJOSICOZGPDY2TYM2H0ROX");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier31() {
	let mut lexer = Lexer::from_string("aL$3skSRj7XMbCN".to_string());
	assert_identifier!(lexer, "AL3SKSRJ7XMBCN");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier32() {
	let mut lexer = Lexer::from_string("uRs0WcYGgy0$MEp9uGITbLmJdosX17kZOTO2BbOMS1h8lf$2Rp74zxs$15Xkl91MuXKmydgqZL4YvPkiQ7B5zStXFIMtYOg7R6ejlCmRfVoYpS$gMbqu8zY".to_string());
	assert_identifier!(lexer, "URS0WCYGGY0MEP9UGITBLMJDOSX17KZOTO2BBOMS1H8LF2RP74ZXS15XKL91MUXKMYDGQZL4YVPKIQ7B5ZSTXFIMTYOG7R6EJLCMRFVOYPSGMBQU8ZY");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier33() {
	let mut lexer = Lexer::from_string("HMIlliUD1vapX7RkKnhrqy5Ovc9Te2U3960qRwpEUzI375GJajWsw22Ok8j$vkMJp6caJQ".to_string());
	assert_identifier!(lexer, "HMILLIUD1VAPX7RKKNHRQY5OVC9TE2U3960QRWPEUZI375GJAJWSW22OK8JVKMJP6CAJQ");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier34() {
	let mut lexer = Lexer::from_string("IAc5whNqcA4AFjI56zn5QrGBvV668KuuoPDedY1FjAPkTyFH3rS8OLH13PfXBh2L3XEPQYq8WXmFhRRZ47X8WJed4JEPYRFjCzsLotq2$".to_string());
	assert_identifier!(lexer, "IAC5WHNQCA4AFJI56ZN5QRGBVV668KUUOPDEDY1FJAPKTYFH3RS8OLH13PFXBH2L3XEPQYQ8WXMFHRRZ47X8WJED4JEPYRFJCZSLOTQ2");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier35() {
	let mut lexer = Lexer::from_string("JprZRv$vS9tKFIVlSPy2KbCv0Df9jxMVTJSGStS6JhQctCNIpcMc82PSlbgsa$3tSsXR".to_string());
	assert_identifier!(lexer, "JPRZRVVS9TKFIVLSPY2KBCV0DF9JXMVTJSGSTS6JHQCTCNIPCMC82PSLBGSA3TSSXR");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier36() {
	let mut lexer = Lexer::from_string("G2$36FHxbQFWYQ4olA92yzYCfx3IRtWEtBMPyEWx3yVA2gIuT".to_string());
	assert_identifier!(lexer, "G236FHXBQFWYQ4OLA92YZYCFX3IRTWETBMPYEWX3YVA2GIUT");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier37() {
	let mut lexer = Lexer::from_string("EdMmo8D9pj1jcQ$6EGnZSrzhc7zQM1AjGF47ozU".to_string());
	assert_identifier!(lexer, "EDMMO8D9PJ1JCQ6EGNZSRZHC7ZQM1AJGF47OZU");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier38() {
	let mut lexer = Lexer::from_string("DbyqAJTkjijfCatJIp0c9gLc83vu1a5NqPClWnC0HPXsj9PhaLWXx6LGqPsDMvE8$OubZp3VuXTFr0UIaHLuz".to_string());
	assert_identifier!(lexer, "DBYQAJTKJIJFCATJIP0C9GLC83VU1A5NQPCLWNC0HPXSJ9PHALWXX6LGQPSDMVE8OUBZP3VUXTFR0UIAHLUZ");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier39() {
	let mut lexer = Lexer::from_string("EK4tweLq20Vglt76tW".to_string());
	assert_identifier!(lexer, "EK4TWELQ20VGLT76TW");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_identifier40() {
	let mut lexer = Lexer::from_string("plv$kY".to_string());
	assert_identifier!(lexer, "PLVKY");
	assert_token!(lexer, None)
}

#[test]
fn test_valid_macro0() {
	let mut lexer = Lexer::from_string("CR".to_string());
	lexer.add_macro("CR".to_string(), 
		Position::zero(),
		"10".to_string()
	);
	assert_token!(lexer, Some((Token::Number(10), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_macro1() {
	let mut lexer = Lexer::from_string("CR LF".to_string());
	lexer.add_macro("CR".to_string(), 
		Position::zero(),
		"10".to_string()
	);
	lexer.add_macro("LF".to_string(), 
		Position::zero(),
		"14".to_string()
	);
	assert_token!(lexer, Some((Token::Number(10), _)));
	assert_token!(lexer, Some((Token::Number(14), _)));
	assert_token!(lexer, None)
}

#[test]
fn test_valid_macro2() {
	let mut lexer = Lexer::from_string("CRLF".to_string());
	lexer.add_macro("CR".to_string(), 
		Position::zero(),
		"10".to_string()
	);
	lexer.add_macro("LF".to_string(), 
		Position::zero(),
		"14".to_string()
	);
	lexer.add_macro("CRLF".to_string(), 
		Position::zero(),
		"CR LF".to_string()
	);
	assert_token!(lexer, Some((Token::Number(10), _)));
	assert_token!(lexer, Some((Token::Number(14), _)));
	assert_token!(lexer, None)
}
