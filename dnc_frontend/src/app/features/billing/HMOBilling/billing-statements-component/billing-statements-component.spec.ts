import { ComponentFixture, TestBed } from '@angular/core/testing';

import { BillingStatementsComponent } from './billing-statements-component';

describe('BillingStatementsComponent', () => {
  let component: BillingStatementsComponent;
  let fixture: ComponentFixture<BillingStatementsComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [BillingStatementsComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(BillingStatementsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
