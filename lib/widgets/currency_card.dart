import 'package:flutter/material.dart';

class CurrencyCard extends StatelessWidget {
  final String name, code, amount;
  final IconData icon;
  final int order;

  final bool isInverted;
  final double _cardOffset = 20;
  final _whiteColor = const Color(0xFFFAFAFA);
  final _blackColor = const Color(0xFF1F2123);

  const CurrencyCard({
    super.key,
    required this.name,
    required this.code,
    required this.amount,
    required this.icon,
    required this.order,
  }) : isInverted = (order % 2 == 0);

  @override
  Widget build(BuildContext context) {
    return Transform.translate(
      offset: Offset(0, -_cardOffset * order),
      child: Container(
        clipBehavior: Clip.hardEdge,
        decoration: BoxDecoration(
          color: isInverted ? _whiteColor : _blackColor,
          borderRadius: const BorderRadius.all(Radius.circular(25)),
        ),
        child: Column(
          children: [
            Padding(
              padding: const EdgeInsets.all(20),
              child: Row(
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        name,
                        style: TextStyle(
                          fontSize: 32,
                          fontWeight: FontWeight.w600,
                          color: isInverted ? _blackColor : _whiteColor,
                        ),
                      ),
                      const SizedBox(height: 10),
                      Row(
                        crossAxisAlignment: CrossAxisAlignment.end,
                        children: [
                          Text(
                            amount,
                            style: TextStyle(
                                fontSize: 20,
                                color: isInverted ? _blackColor : _whiteColor),
                          ),
                          const SizedBox(
                            width: 5,
                          ),
                          Text(
                            code,
                            style: TextStyle(
                              color: (isInverted ? _blackColor : _whiteColor)
                                  .withOpacity(0.8),
                            ),
                          ),
                        ],
                      ),
                    ],
                  ),
                  Transform.scale(
                    scale: 2,
                    child: Transform.translate(
                      offset: const Offset(-5, 10),
                      child: Icon(
                        icon,
                        color: isInverted ? _blackColor : _whiteColor,
                        size: 88,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}
