import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupDentistContracts } from './setup-dentist-contracts';

describe('SetupDentistContracts', () => {
  let component: SetupDentistContracts;
  let fixture: ComponentFixture<SetupDentistContracts>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupDentistContracts]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupDentistContracts);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
