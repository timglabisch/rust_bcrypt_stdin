<?php

foreach (range(0,1000) as $i) {
  $options = [
      'cost' => 9,
      'salt' => uniqid().uniqid(),
  ];

  echo password_hash("foo".$i, PASSWORD_BCRYPT, $options)."\n";
}
