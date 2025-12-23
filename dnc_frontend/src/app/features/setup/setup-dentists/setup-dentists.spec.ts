import { ComponentFixture, TestBed } from '@angular/core/testing';

import { SetupDentists } from './setup-dentists';

describe('SetupDentists', () => {
  let component: SetupDentists;
  let fixture: ComponentFixture<SetupDentists>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [SetupDentists]
    })
    .compileComponents();

    fixture = TestBed.createComponent(SetupDentists);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
