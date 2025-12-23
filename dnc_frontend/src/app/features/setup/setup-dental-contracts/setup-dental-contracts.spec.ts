import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupDentalContracts } from './setup-dental-contracts';

describe('SetupDentalContracts', () => {
  let component: SetupDentalContracts;
  let fixture: ComponentFixture<SetupDentalContracts>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupDentalContracts]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupDentalContracts);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
