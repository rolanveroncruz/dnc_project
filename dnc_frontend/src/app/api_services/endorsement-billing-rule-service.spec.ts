import { TestBed } from '@angular/core/testing';

import { EndorsementBillingRuleService } from './endorsement-billing-rule-service';

describe('EndorsementBillingRuleService', () => {
  let service: EndorsementBillingRuleService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(EndorsementBillingRuleService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
