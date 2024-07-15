import 'package:flutter/material.dart';
import 'package:flutter_toonflix/screens/home_screen.dart';

class ToonflixApp extends StatelessWidget {
  const ToonflixApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      home: HomeScreen(),
    );
  }
}
